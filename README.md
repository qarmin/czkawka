# Czkawka
Czkawka is simple and easy to use alternative to Fslint written in Rust.  
It is in very early development, so most of the functions aren't added and doesn't work.  


## Done
- Basic menu(need refactoring)
- Duplicated file finding - CLI
  - Including and excluding directories(absolute pathes)
  - Option to remove files in different ways
  - Fast(by size) or accurate(by hash) file checking

## TODO
- Duplicated file finding - CLI
  - saving results to file
  - support for * when excluding files and folders
- GUI(GTK)
- Removing empty folders
- Files with debug symbols
- Support for showing only duplicates with specific extension, name(Regex support needed)
- Maybe windows support, but this will need some refactoring in code

## Usage
- Install requirements for GTK(minimum 3.16)
```
apt install -y libgtk-3-dev
```
- Download source
```
git clone github/czkawka // TODO
cd czkawka
```
- Run GUI(Still WIP)
```
cargo run --bin czkawka_gui
```
- Run CLI



## License
Czkawka is released under the terms of the GNU Lesser General Public License, version 2.1 or, at your option, any later version, as published by the Free Software Foundation. 