use std::io::Cursor;

#[cfg(feature = "audio")]
use rodio::{Decoder, OutputStream, Sink};

// Default embedded audio file
const DEFAULT_STOP_AUDIO: &[u8] = include_bytes!("../audio/stop_bit.m4a");

// Audio player that can play sound effects
pub struct AudioPlayer {
    #[cfg(feature = "audio")]
    audio_data: Vec<u8>,
}

impl AudioPlayer {
    /// Create a new audio player, loading custom audio file from environment variable if available
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
        // Try to load custom audio file from environment variable
        if let Ok(custom_path) = std::env::var("KROKIET_AUDIO_STOP_FILE") {
            match std::fs::read(&custom_path) {
                Ok(data) => {
                    // Validate that the audio file can be decoded
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

        // Fall back to default embedded audio
        DEFAULT_STOP_AUDIO.to_vec()
    }

    /// Play the scan completion sound
    pub fn play_scan_completed(&self) {
        #[cfg(feature = "audio")]
        {
            // Spawn a thread to play the audio so it doesn't block
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
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|e| format!("Failed to get audio output stream: {}", e))?;

        // Create a sink to play audio
        let sink = Sink::try_new(&stream_handle).map_err(|e| format!("Failed to create audio sink: {}", e))?;

        // Decode the audio file
        let cursor = Cursor::new(audio_data);
        let source = Decoder::new(cursor).map_err(|e| format!("Failed to decode audio: {}", e))?;

        // Play the audio
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
