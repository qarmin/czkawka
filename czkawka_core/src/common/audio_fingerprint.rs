use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use log::error;
use rusty_chromaprint::{Configuration, Fingerprinter};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

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

        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(|_| "unsupported format".to_string())?;

        let mut format = probed.format;

        // Select the first non-null track that has a sample rate – this covers both pure audio
        // files and video containers where `channels` may not be populated in the header but
        // becomes available after decoding the first packet.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL && t.codec_params.sample_rate.is_some())
            .ok_or_else(|| "no supported audio track".to_string())?;

        let dec_opts: DecoderOptions = Default::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|_| "unsupported codec".to_string())?;

        let track_id = track.id;

        let mut printer = Fingerprinter::new(config);
        // `printer` is started lazily on the first decoded packet so we can read the real
        // channel count from the audio buffer spec even when the container header omits it.
        let mut printer_started = false;

        let mut sample_buf: Option<SampleBuffer<i16>> = None;
        // total interleaved samples (all channels combined), used to derive duration
        let mut total_interleaved_samples: u64 = 0;
        let mut audio_channels: u32 = 0;
        let mut audio_sample_rate: u32 = 0;
        let mut sum_sq: f64 = 0.0;
        let mut max_amp: f64 = 0.0;

        while let Ok(packet) = format.next_packet() {
            if check_if_stop_received(stop_flag) {
                return Ok(None);
            }

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    let spec = *audio_buf.spec();

                    if sample_buf.is_none() {
                        let duration = audio_buf.capacity() as u64;
                        sample_buf = Some(SampleBuffer::<i16>::new(duration, spec));
                    }

                    if !printer_started {
                        audio_sample_rate = spec.rate;
                        audio_channels = spec.channels.count() as u32;
                        printer.start(audio_sample_rate, audio_channels).map_err(|_| "initializing fingerprinter".to_string())?;
                        printer_started = true;
                    }

                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(audio_buf);
                        let samples = buf.samples();
                        total_interleaved_samples += samples.len() as u64;
                        for &s in samples {
                            let v = f64::from(s) / f64::from(i16::MAX);
                            sum_sq += v * v;
                            let a = v.abs();
                            if a > max_amp {
                                max_amp = a;
                            }
                        }
                        printer.consume(samples);
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => (),
                Err(_) => break,
            }
        }

        if !printer_started {
            return Err("no audio frames decoded".to_string());
        }

        printer.finish();
        let fingerprint = printer.fingerprint().to_vec();

        // Derive duration from the count of decoded samples
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
