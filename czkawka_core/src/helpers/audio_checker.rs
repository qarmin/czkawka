use std::fs::File;
use std::io;

use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error;
use symphonia::core::errors::Error::IoError;
use symphonia::core::io::MediaSourceStream;

pub fn parse_audio_file(file_handler: File) -> Result<(), Error> {
    let mss = MediaSourceStream::new(Box::new(file_handler), Default::default());

    let Ok(probed) = symphonia::default::get_probe().format(&Default::default(), mss, &Default::default(), &Default::default()) else {
        return Err(Error::Unsupported("probe info not available/file not recognized"));
    };

    let mut format = probed.format;

    let Some(track) = format.tracks().iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL) else {
        return Err(Error::Unsupported("not supported audio track"));
    };

    let Ok(mut decoder) = symphonia::default::get_codecs().make(&track.codec_params, &Default::default()) else {
        return Err(Error::Unsupported("not supported codec"));
    };

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                return Err(Error::ResetRequired);
            }
            Err(err) => {
                if let IoError(ref er) = err {
                    // Catch eof, not sure how to do it properly
                    if er.kind() == io::ErrorKind::UnexpectedEof {
                        return Ok(());
                    }
                }
                return Err(err);
            }
        };

        decoder.decode(&packet)?;
    }
}
