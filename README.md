# Czkawka
Czkawka is simple and easy to use alternative to Fslint written in Rust.  
It is in very early development, so most of the functions aren't added and doesn't work.  


## Done
- Rich instruction with examples - CLI(`cargo run --bin czkawka_cli`)
- Duplicated file finding - CLI
  - Including and excluding directories(absolute pathes)
  - Option to remove files in different ways
  - Fast(by size) or accurate(by hash) file checking
- Empty folders finding - CLI
  - Advanced empty files finding(finds and remove folders which contains only empty folders)
  - Option to remove all files

## TODO
- Comments - a lot of things should be described
- Tests
  - Github CI
  - Unit tests(if available)
- Duplicated file finding - CLI
  - saving results to file
  - support for * when excluding files and folders
- GUI(GTK)
- Files with debug symbols
- Support for showing only duplicates with specific extension, name(Regex support needed)
- Maybe windows support, but this will need some refactoring in code

## Usage
For now only Linux(and probably also macOS) is supported
- Install requirements for GTK(minimum 3.16)
```
apt install -y libgtk-3-dev
```
- Download source
```
git clone https://github.com/qarmin/czkawka.git
cd czkawka
```
- Run GUI(Still WIP)
```
cargo run --bin czkawka_gui
```
- Run CLI
```
cargo run --bin czkawka_cli
```

## License
Czkawka is released under the terms of the GNU Lesser General Public License, version 2.1 or, at your option, any later version, as published by the Free Software Foundation. 