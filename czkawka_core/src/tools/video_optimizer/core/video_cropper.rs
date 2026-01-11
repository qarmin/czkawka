use log::debug;

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::{VideoCropEntry, VideoCropParams};

// Zaimplementuj cropopwanie, jako przyjemny do czytania i testów funkcjonalność
//
// To ma:
// - znajdywać w przybliżeniu początek i koniec sensowny wideo(jeśli np. przez 5 sekund jest ten sam obraz, to znaczy że to intro/outro i można usunąć)
// - ma znajdywać czarne pasy na górze/dole/lewej prawje strony(w przyszłości tobędą statyczne części video, ale obecnie tylko czarne pasy)
//
// Algorythm ma działać tak
// - masz duration i działasz na nim
// Pobierasz z grubsza pierwszą klatkę, ostatnią klatkę - w sensie z grubsza, bo nie chcesz dekodować całego wideo klatka po klatce
// Jeśli tutaj nie ma ani czarnych pasów to odpuszczas krok sprawdzania czarnych pasów
// Ale dalej musisz sprawdzic czy pocztek i koniec mogą być przycięte
// Przycięcie początku i końca, powinno być przycięte do około 0.5 sekundy dokładności - czyli 0s - obraz A, 0.5s - obraz A, 1s - obraz B, to znaczy że można przyciąć do 0.5s
// Analogicznie na końcu wideo
// W obu przypadkach, najlepiej skorzystać z wyszukiwania binarnego - ale tylko po początkowych krokach
// A początkowe kroki to(dla dużych video, bo dla małych to będzie przetwarzane szybkeij):
// Sprawdź pierwszą i ostantią klatkę, sprawdź potem w krokach 5s, 30s, 100s, 300s - jeśli znajdziemy rożnicę, to suzkamy binarnie pomiedzy np. 30 - 100, czyli 65 etc.


pub fn check_video_crop(mut entry: VideoCropEntry, params: &VideoCropParams) -> VideoCropEntry {
    debug!("Checking video for crop: {}", entry.path.display());

    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(e);
            return entry;
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some("Failed to get video codec".to_string());
        return entry;
    };

    entry.codec = current_codec;
    match (metadata.width, metadata.height) {
        (Some(width), Some(height)) => {
            entry.width = width;
            entry.height = height;
        }
        _ => {
            entry.error = Some("Failed to get video dimensions".to_string());
            return entry;
        }
    }

    // TODO: Implement crop detection logic - split extracting metadata from real crop detection
    // For now, just return the entry with basic metadata

    entry
}
