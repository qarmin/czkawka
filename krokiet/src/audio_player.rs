use std::io::Cursor;

#[cfg(feature = "audio")]
use rodio::{Decoder, OutputStreamBuilder, Sink};

const DEFAULT_STOP_AUDIO: &[u8] = include_bytes!("../audio/stop_bit.m4a");

pub struct AudioPlayer {
    #[cfg(feature = "audio")]
    audio_data: Vec<u8>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            let audio_data = Self::load_audio_data();
            Self { audio_data }
        }
        #[cfg(not(feature = "audio"))]
        {
            Self {}
        }
    }

    #[cfg(feature = "audio")]
    fn load_audio_data() -> Vec<u8> {
        if let Ok(custom_path) = std::env::var("KROKIET_AUDIO_STOP_FILE") {
            match std::fs::read(&custom_path) {
                Ok(data) => {
                    let cursor = Cursor::new(data.clone());
                    match Decoder::new(cursor) {
                        Ok(_) => {
                            log::info!("Loaded custom audio file from: {}", custom_path);
                            return data;
                        }
                        Err(e) => {
                            log::error!("Failed to decode custom audio file from {}: {}", custom_path, e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to read custom audio file from {}: {}", custom_path, e);
                }
            }
        }

        DEFAULT_STOP_AUDIO.to_vec()
    }

    pub fn play_scan_completed(&self) {
        #[cfg(feature = "audio")]
        {
            let audio_data = self.audio_data.clone();
            std::thread::spawn(move || {
                if let Err(e) = Self::play_audio_blocking(&audio_data) {
                    log::error!("Failed to play scan completion audio: {}", e);
                }
            });
        }
        #[cfg(not(feature = "audio"))]
        {
            // No-op when audio feature is disabled
        }
    }

    #[cfg(feature = "audio")]
    fn play_audio_blocking(audio_data: &[u8]) -> Result<(), String> {
        let stream_handle = OutputStreamBuilder::open_default_stream()
            .map_err(|e| format!("Failed to get audio output stream: {}", e))?;

        let sink = Sink::connect_new(&stream_handle.mixer());

        let cursor = Cursor::new(audio_data.to_vec());
        let source = Decoder::new(cursor).map_err(|e| format!("Failed to decode audio: {}", e))?;

        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
