# Czkawka
Czkawka is simple, fast and easy to use alternative to Fslint, written in Rust.  
This is my first ever project in Rust so probably a lot of things are not being written in the most optimal way.

## Why?
In internet exists a lot of tools to find duplicates, empty folders, temporary files etc. but in most cases there are only available on CLI, which is hard to use by users.  
GUI FSlint allows to really easy select different files and folders, but is based on old and unsupported Python 2 and GTK 2.  
Other tools are mostly written in C/C++ to provide big performance but still needs to be tested a lot if not contains memory leaks, invalid memory reads/write and double frees.

## Features
- Written in fast and memory safe Rust
- CLI frontend, very fast and powerful with rich help
- GUI GTK frontend - use modern GTK 3 and looks similar to FSlint
- GUI Orbtk frontend(Very early WIP) - alternative GUI with reduced functionality
- Saving results to file - allows to easily read entries found by tool
- Rich search option - allows setting absolute included and excluded directories, set of allowed files extensions or excluded items with * wildcard
- Clean Glade file in which UI can be easily modernized
- Multiple tools to use:
  - Duplicates - Finds duplicates basing on its size(fast), hash(accurate), first 1MB of hash(moderate)
  - Empty Folders - Finds empty folders with help of advanced algorithm
  - Big Files - Finds provided number of the biggest files in given location
  - Empty Files - Looks for empty files across disk
  - Temporary Files - Allows finding temporary files

## Usage and requirements
Rust 1.46 - probably lower also works fine  
GTK 3.24 - for GTK backend

Precompiled binaries are here(may not work in every Linux distro) - https://github.com/qarmin/czkawka/releases/

For now only Linux(and maybe also macOS) is supported

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
![GUI GTK](https://user-images.githubusercontent.com/41945903/94106023-d72f9700-fe3a-11ea-821d-48484afd74fb.png)
- Run alternative Orbtk GUI(Still WIP, and currently stopped due https://github.com/intellij-rust/intellij-rust/issues/5943)
```
cargo run --bin czkawka_gui_orbtk
```
![GUI Orbtk](https://user-images.githubusercontent.com/41945903/92405241-7b27fb80-f135-11ea-9fc4-5ebc2b76b011.png)
- Run CLI(this will print help with a lot of examples)
```
cargo run --bin czkawka_cli
```
![CLI](https://user-images.githubusercontent.com/41945903/93716816-0bbcfd80-fb72-11ea-8d31-4c87cc2abe6d.png)

## Speed
Since Czkawka is written in Rust and aims to be a faster alternative for written in Python - FSlint we need to compare speed of this two tools.

I checked my home directory without any folder exceptions(I removed all directories from FSlint advanced tab) which contained 379359 files and 42445 folders and 50301 duplicated files in 29723 groups which took 450,4 MB.

First run reads file entry and save it to cache so this step is mostly limited by disk performance, and with second run cache helps it so searching is a lot of faster.

Duplicate Checker(Version 0.1.0)

| App| Executing Time |
|:----------:|:-------------:|
| Fslint (First Run)| 140s |
| Fslint (Second Run)| 23s |
| Czkawka CLI Release(First Run) | 128s |
| Czkawka CLI Release(Second Run) | 8s |

| App| Idle Ram | Max Operational Ram Usage |
|:----------:|:-------------:|:-------------:|
| Fslint |  |  |
| Czkawka CLI Release |  |
| Czkawka GTK GUI Release |  |


Empty folder finder

| App| Executing Time |
|:----------:|:-------------:|
| Fslint |  |
| Czkawka CLI Release |  |
| Czkawka GTK GUI Release |  |

Differences should be more visible when using slower processor or faster disk.

## How it works?
### Duplicate Finder
The only required parameter for checking duplicates is included folders `-i`. This parameter validates provided folders - which must have absolute path(without ~ and other similar symbols at the beginning), not contains *(wildcard), be dir(not file or symlink), exists. Later same things are done with excluded folders `-e`. 

Next, this included and excluded folders are optimized due to tree structure of file system:
- Folders which contains another folders are combined(separately for included and excluded) - `/home/pulpet` and `/home/pulpet/a` are combined to `/home/pulpet`
- Included folders which are located inside excluded ones are delete - Included folder `/etc/tomcat/` is deleted because excluded folder is `/etc/`
- Non existed directories are being removed
- Excluded path which are outside included path are deleted - Excluded path `/etc/` is removed if included path is `/home/`
If after optimization there is no included folders, then program ends with non zero value(TODO, this should be handled by returning value).

Next with provided by user minimal size of checked size `-s`, program checks recursively or not included folders and checks files by sizes and put files with same sizes to different boxes. 
Next boxes which contains only one element are removed because files inside that means that are not duplicated.

Now if user select this, then provided is checking hash of file, because may happens that files have equal size, but differ in one or more bytes.

There are two available methods to check hash:
- full(default) - check hash of entire file so this method is slow especially with large files and but there is almost no chance that two different files will be treated like they were a duplicates.
- partial - check hash only max first 1MB of file, so it is a lot of more accurate than only checking size of files, but still there is very small chance that files which were identified as duplicates they are not.

At the end if user used `-delete` option, specified files are removed - All Except Oldest/Newest or Only Oldest/Newest
 
## Name
Czkawka is a Polish word which means hiccup.  
I chose this name because I wanted to hear people speaking other languages pronounce it.

## License
Code is distributed under MIT license.

Program is completely free to use.

"Gratis to uczciwa cena" - "Free is a fair price"