use std::fs::File;
use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use symphonia::core::codecs::CodecParameters;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::errors::Error::IoError;
use symphonia::core::io::MediaSourceStream;

use crate::common::progress_stop_handler::check_if_stop_received;

#[derive(Debug)]
pub enum AudioCheckError {
    UnsupportedCodec,
    Other(String),
}

impl AudioCheckError {
    fn other(err: &Error) -> Self {
        Self::Other(err.to_string())
    }
}

pub fn parse_audio_file(file_handler: File, stop_flag: &Arc<AtomicBool>) -> Result<Option<()>, AudioCheckError> {
    let mss = MediaSourceStream::new(Box::new(file_handler), Default::default());

    let Ok(mut format) = symphonia::default::get_probe().probe(&Default::default(), mss, Default::default(), Default::default()) else {
        return Err(AudioCheckError::Other("probe info not available/file not recognized".to_string()));
    };

    let Some(track) = format.tracks().iter().find(|t| matches!(t.codec_params.as_ref(), Some(CodecParameters::Audio(_)))) else {
        return Err(AudioCheckError::Other("not supported audio track".to_string()));
    };

    let audio_params = match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(p)) => p.clone(),
        _ => unreachable!(),
    };

    let Ok(mut decoder) = symphonia::default::get_codecs().make_audio_decoder(&audio_params, &AudioDecoderOptions::default()) else {
        return Err(AudioCheckError::UnsupportedCodec);
    };

    loop {
        if check_if_stop_received(stop_flag) {
            return Ok(None);
        }

        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => return Ok(Some(())),
            Err(err @ Error::ResetRequired) => {
                return Err(AudioCheckError::other(&err));
            }
            Err(err) => {
                if let IoError(ref er) = err
                    && er.kind() == io::ErrorKind::UnexpectedEof
                {
                    return Ok(Some(()));
                }
                return Err(AudioCheckError::other(&err));
            }
        };

        if let Err(err) = decoder.decode(&packet) {
            return Err(AudioCheckError::other(&err));
        }
    }
}
