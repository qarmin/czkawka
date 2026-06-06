use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use log::error;
use rusty_chromaprint::{Configuration, Fingerprinter};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

use crate::common::create_crash_message;
use crate::common::progress_stop_handler::check_if_stop_received;

/// Compute a Chromaprint fingerprint for the first audio track found in `path`.
///
/// Works for both pure audio files and video containers. Channel count is read from the
/// decoded audio buffer spec rather than the container header, because some encoders
/// (e.g. AAC in MP4) omit channel info from the header.
///
/// Returns:
/// - `Ok(Some((fingerprint, duration_seconds)))` on success
/// - `Ok(None)` if the stop flag was raised during processing
/// - `Err(message)` on processing failure or if no audio track is found
pub(crate) fn calc_fingerprint_and_duration<P: AsRef<Path>>(path: P, config: &Configuration, stop_flag: &Arc<AtomicBool>) -> Result<Option<(Vec<u32>, u32)>, String> {
    let path = path.as_ref().to_path_buf();
    std::panic::catch_unwind(|| {
        let path = &path;

        let src = File::open(path).map_err(|_| "failed to open file".to_string())?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
            hint.with_extension(ext);
        }

        let mut format = symphonia::default::get_probe()
            .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
            .map_err(|_| "unsupported format".to_string())?;

        let track = format
            .tracks()
            .iter()
            .find(|t| {
                if let Some(CodecParameters::Audio(p)) = t.codec_params.as_ref() {
                    p.sample_rate.is_some()
                } else {
                    false
                }
            })
            .ok_or_else(|| "no supported audio track".to_string())?;

        let audio_params = match track.codec_params.as_ref() {
            Some(CodecParameters::Audio(p)) => p.clone(),
            _ => unreachable!(),
        };
        let track_id = track.id;

        let dec_opts = AudioDecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(&audio_params, &dec_opts)
            .map_err(|_| "unsupported codec".to_string())?;

        let mut printer = Fingerprinter::new(config);
        let mut printer_started = false;

        let mut samples_i16: Vec<i16> = Vec::new();
        let mut total_interleaved_samples: u64 = 0;
        let mut audio_channels: u32 = 0;
        let mut audio_sample_rate: u32 = 0;
        let mut sum_sq: f64 = 0.0;
        let mut max_amp: f64 = 0.0;

        loop {
            if check_if_stop_received(stop_flag) {
                return Ok(None);
            }

            let packet = match format.next_packet() {
                Ok(Some(packet)) => packet,
                Ok(None) => break,
                Err(symphonia::core::errors::Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(format!("error while reading audio packet: {e}")),
            };

            if packet.track_id != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    let spec = audio_buf.spec();

                    if !printer_started {
                        audio_sample_rate = spec.rate();
                        audio_channels = spec.channels().count() as u32;
                        printer.start(audio_sample_rate, audio_channels).map_err(|_| "initializing fingerprinter".to_string())?;
                        printer_started = true;
                    }

                    samples_i16.clear();
                    audio_buf.copy_to_vec_interleaved(&mut samples_i16);

                    total_interleaved_samples += samples_i16.len() as u64;
                    for &s in &samples_i16 {
                        let v = f64::from(s) / f64::from(i16::MAX);
                        sum_sq += v * v;
                        let a = v.abs();
                        if a > max_amp {
                            max_amp = a;
                        }
                    }
                    printer.consume(&samples_i16);
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => (),
                Err(e) => return Err(format!("fatal error while decoding audio: {e}")),
            }
        }

        if !printer_started {
            return Err("no audio frames decoded".to_string());
        }

        printer.finish();
        let fingerprint = printer.fingerprint().to_vec();

        let duration_seconds = if audio_channels > 0 && audio_sample_rate > 0 {
            (total_interleaved_samples / u64::from(audio_channels) / u64::from(audio_sample_rate)) as u32
        } else {
            0
        };

        let rms = if total_interleaved_samples > 0 {
            (sum_sq / total_interleaved_samples as f64).sqrt()
        } else {
            0.0
        };
        if rms < 0.001 && max_amp < 0.01 {
            // Cache with an empty fingerprint so this file is not re-decoded on the next run
            // but is still excluded from comparisons via the `!fingerprint.is_empty()` filter.
            return Ok(Some((vec![], duration_seconds)));
        }

        Ok(Some((fingerprint, duration_seconds)))
    })
    .unwrap_or_else(|_| {
        let message = create_crash_message("Symphonia", &path.to_string_lossy(), "https://github.com/pdeljanov/Symphonia");
        error!("{message}");
        Err(message)
    })
}
