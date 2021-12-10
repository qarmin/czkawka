potato-error = potato errorrer


# Main window
music_title_checkbox = Title
music_artist_checkbox = Artist
music_album_title_checkbox = Album Title
music_album_artist_checkbox = Album Artist
music_year_checkbox = Year
music_comparison_checkbox = Approximate Comparison

duplicate_mode_name_checkbox = Name
duplicate_mode_size_checkbox = Size
duplicate_mode_hash_checkbox = Hash

duplicate_mode_name_checkbox_tooltip = 
        Finds files which have same name.
  
        This mode not checking what file contain inside, so be carefully when using it.
        
duplicate_mode_size_checkbox_tooltip = 
        Finds files which have same size.

        This mode not checking what file contain inside, so be carefully when using it.
        
duplicate_mode_hash_checkbox_tooltip = 
        Finds files which have the same content.
  
        This mode hashes file and later compare this hashes to find duplicates.
  
        Tool heavily uses cache, so second and further scans of same data should be a lot of faster that first.

duplicate_hash_checkbox_blake3 = Blake3 is cryptographic hash function. It is used as default hash algorithm, because it is very fast.
duplicate_hash_checkbox_crc32 = CRC32 is simple hash function. It should be faster than Blake3, but probably may have very rarely some collisions.
duplicate_hash_checkbox_xxh3 = XXH3 is very similar in case of performance and hash quality to Blake3, so such modes can be easily used.

image_hash_checkbox_8 = Default hash size, with very high similarity it produce quite good results and don't save too much data too cache.
image_hash_checkbox_16 = More precise than 8, so can be used to find very similar pictures, but create bigger cache entries.
image_hash_checkbox_32 = Hash of this size provide very big similarity which is more than enough for most usages.
image_hash_checkbox_64 = Paranoid mode, such tool create really big cache files and will catch almost same images.









