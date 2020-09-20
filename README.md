# Czkawka
Czkawka is simple, fast and easy to use alternative to Fslint, written in Rust.  
It is in very early development, so most of the functions aren't added and doesn't work.  
This is my first ever project in Rust so probably a lot of things are written in the most optimal way.

## Done
- Rich instruction with examples - CLI(`cargo run --bin czkawka_cli`)
- GTK Frontend(Still WIP) - (`cargo run --bin czkawka_gui`)
- Orbtk Frontend(Still very early WIP) - (`cargo run --bin czkawka_gui_orbtk`)
- Saving results to file
- Duplicated file finding
  - Including and excluding directories(absolute paths)
  - Option to remove all except newest, oldest and one oldest or newest
  - Fast(by size) or accurate(by hash) file checking
  - Support for * wildcard when excluding files and folders
  - Checking only selected extensions(default macros like IMAGE, VIDEO, TEXT are available)
- Empty folders finding
  - Advanced empty files finding(finds and remove folders which contains only empty folders)
  - Option to remove all files

## TODO
- Comments - a lot of things should be described
- Github CI
- More unit tests
- Finding files with debug symbols
- Maybe windows support, but this will need some refactoring in code
- Translation support
- Add support for fast searching based on checking only first ~1MB of file.

## Usage and requirements
Rustc 1.46 works fine(not sure about a minimal version)  
GTK 3.18 - for GTK backend

For now only Linux(and probably also macOS) is supported

- Install requirements for GTK
```
apt install -y libgtk-3-dev
```

- Download source
```
git clone https://github.com/qarmin/czkawka.git
cd czkawka
```
- Run GTK GUI(Still WIP)
```
cargo run --bin czkawka_gui
```
![GUI GTK](https://user-images.githubusercontent.com/41945903/93716818-0eb7ee00-fb72-11ea-8fdb-50b49a221868.png)
- Run alternative Orbtk GUI(Still WIP)
```
cargo run --bin czkawka_gui_orbtk
```
![GUI Orbtk](https://user-images.githubusercontent.com/41945903/92405241-7b27fb80-f135-11ea-9fc4-5ebc2b76b011.png)
- Run CLI
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

## How it works?
### Duplicate Finder
The only required parameter for checking duplicates is included folders `-i`. This parameter validates provided folders - which must have absolute path(without ~ and other similar symbols at the beginning), not contains *(wildcard), be dir(not file or symlink), exists. Later same things are done with excluded folders `-e`. 

Next, this included and excluded folders are optimized due to tree structure of file system:
- Folders which contains another folders are combined(separately for included and excluded) - `/home/pulpet` and `/home/pulpet/a` are combined to `/home/pulpet`
- Included folders which are located inside excluded ones are delete - Included folder `/etc/tomcat/` is deleted because excluded folder is `/etc/`
- Non existed directories are being removed
- Excluded path which are outside include path are deleted - Exclude path `/etc/` is removed if included path is `/home/`
If after optimization there is no include folders, then program ends with non zero value(TODO, this should be handled by returning value).

Next with provided by user minimal size of checked size `-s`, program checks recursively(TODO should be an option to turn off a recursion) included folders and checks files by sizes and put it files with same sizes to different boxes. 
Next boxes which contains only one element are removed because files inside are not duplicated.

Next by default also is checked hash to be sure that files with equal size are identical.

## Speed
Since Czkawka is written in Rust and aims to be a faster alternative for written in Python - FSlint we need to compare speed of this two tools.

I checked my home directory without any folder exceptions(I removed all directories from FSlint advanced tab) which contained 379359 files and 42445 folders and 50301 duplicated files in 29723 groups which took 450,4 MB.

First run reads file entry and save it to cache so this step is mostly limited by disk performance, and with second run cache helps it so searching is a lot of faster.

Duplicate Checker(Version 0.1)

| App| Executing Time |
|:----------:|:-------------:|
| Fslint (First Run)| 140s |
| Fslint (Second Run)| 23s |
| Czkawka CLI Release(First Run) | 128s |
| Czkawka CLI Release(Second Run) | 8s |

| App| Idle Ram | Max Operational Ram Usage |
|:----------:|:-------------:|:-------------:|
| Fslint |  |  |
| Czkawka CLI Debug |  |
| Czkawka CLI Release |  |
| Czkawka GUI Debug |  |
| Czkawka GUI Release |  |


Empty folder finder

| App| Executing Time |
|:----------:|:-------------:|
| Fslint |  |
| Czkawka CLI Debug |  |
| Czkawka CLI Release |  |

Differences should be more visible when using slower processor or faster disk.

## License
Code is distributed under MIT license.
