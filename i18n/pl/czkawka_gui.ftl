potato-error = Heheszek

# Main window
music_title_checkbox = Tytuł
music_artist_checkbox = Wykonawca
music_album_title_checkbox = Tytuł Albumu
music_album_artist_checkbox = Wykonawca Albumu
music_year_checkbox = Rok
music_comparison_checkbox = Przybliżone Porównywanie

duplicate_mode_name_checkbox = Nazwa
duplicate_mode_size_checkbox = Rozmiar
duplicate_mode_hash_checkbox = Hash


duplicate_mode_name_checkbox_tooltip = 
        Służy do znajdowania plików o identycznych nazwach.
        
        Ten tryb nie sprawdza zawartości pliku, dlatego należy uważać przy jego stosowaniu.
        
duplicate_mode_size_checkbox_tooltip = 
        Służy do znajdowania plików o identycznych rozmiarach.

        Ten tryb nie sprawdza zawartości pliku, dlatego należy uważać przy jego stosowaniu.
        
duplicate_mode_hash_checkbox_tooltip = 
        Znajduje pliki z identyczną zawartością, bez względu na nazwę i rozszerzenie pliku.

        W tym trybie, każdy plik jest hasowany a następnie każdy hash jest porównywany z innymi.
  
        Ten tryb używa pamięci podręcznej do przechowywania raz obliczonych hashy, dlatego drugie i kolejne skanowanie, powinno być o wiele szybsze niż pierwsze.

duplicate_hash_checkbox_blake3 = Blake3 jest kryptograficzną funkcją haszującą. Z racji połączenia szybkości i niskiej ilości kolizji jest to domyślny tryb.
duplicate_hash_checkbox_crc32 = CRC32 to prosta funkcja haszująca. Powinna być szybsza niż Blake3, lecz bardzo rzadko może mogą wystąpić kolizje hashy.
duplicate_hash_checkbox_xxh3 = XXH3 zarówno pod względem jakości hashu jak i wydajności jest podobny do Blake3, dlatego te algorytmy mogą być używane wymiennie.

image_hash_checkbox_8 = Domyślna wielkość hasha, pozwala na wyszukanie obrazów od bardzo do minimalnie podobnych. Przy niższych poziomach podobieństwa, może grupować wiele niepodobnych obrazków.
image_hash_checkbox_16 = Bardziej precyzyjny od 8. Przydatny gdy ze względu na dużą ilość zdjęć i kolizji między hashami niższy hash wyświetla nieprawidłowe dane.
image_hash_checkbox_32 = Umożliwia wyszukiwanie niemal identycznych obrazów, różniących się niewielkimi szczegółami.
image_hash_checkbox_64 = Tryb predatora, tworzy ogromne hashe które porównywane, wczytywane i zapisywane znacznie dłużej od poprzedników. Z racji wad nie powinien być raczej stosowany.
