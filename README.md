# Czkawka
Czkawka is simple and easy to use alternative to Fslint written in Rust.  
It is in very early development, so most of the functions aren't added and doesn't work.  
This is my first ever project in Rust so probably a lot of things are written in the most optimal way.

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
- Alternative GUI with orbtk
- Finding files with debug symbols
- Support for showing only duplicates with specific extension, name(Regex support needed)
- Maybe windows support, but this will need some refactoring in code
- Translation support

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

## How it works?
### Duplicate Finder
The only required parameter for checking duplicates is included folders `-i`. This parameter validates provided folders - which must have absolute path(without ~ and other similar symbols at the beginning),  not contains *(wildcard), be dir(not file or symlink), exists. Later same things are done with excluded folders `-e`.  
Next, this included and excluded folders are optimized due to tree structure of file system:
- Folders which contains another folders are combined(separately for included and excluded) - `/home/pulpet` and `/home/pulpet/a` are combined to `/home/pulpet`
- Inlcuded folders which are located inside excluded ones are delete - Included folder `/etc/tomcat/` is deleted because excluded folder is `/etc/`
- Non existed directories are removed
- Excluded path which are outside include path are deleted - Exclude path `/etc/` is removed if included path is `/home/`
If after optimization there is no include folders, then program ends with non zero value(TODO, this should be handled by returning value).
Next with provided by user minimal size of checked size `-s`, program checks requsively(TODO should be an option to turn off a recursion)  included folders and checks files by sizes and put it files with same sizes to different boxes. 
Next boxes which contains only one element are removed because files inside are not duplicated.
Next by default also is checked hash to get 

## License
Code is distributed under MIT license.
