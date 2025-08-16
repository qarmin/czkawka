# Czkawka CLI

CLI frontend that allows you to use Czkawka from the terminal.

## Requirements

Precompiled binaries should work without any additional dependencies on Linux (Ubuntu 22.04+), Windows (10+), and macOS (10.15+).

On Linux, it is even possible (with eyra) to avoid libc entirely and use a fully static Rust binary, but alternatively you can use musl for this task.

If you want to use the similar videos tool, you need to install ffmpeg (runtime dependency).  
If you want to use heif/libraw/libavif (build/runtime dependency), you need to install the required packages.

- macOS: `brew install ffmpeg libraw libheif libavif dav1d` – [ffmpeg formula](https://formulae.brew.sh/formula/ffmpeg)
- Linux: `sudo apt install ffmpeg libraw-dev libheif-dev libavif-dev libdav1d-dev`
- Windows: `choco install ffmpeg` – or, if not working, download from [ffmpeg.org](https://ffmpeg.org/download.html#build-windows) and
  unpack to the location with `czkawka_cli.exe`. Heif and libraw are not supported on Windows.

## Compilation

To compile, you need to have Rust installed via [rustup](https://rustup.rs/). Then, build with:

```shell
cargo run --release --bin czkawka_cli
```

You can enable additional features with:

```shell
cargo run --release --bin czkawka_cli --features "heif,libraw,libavif"
```

## How to use

The application includes concise help for each tool, which you can display by running:
```
czkawka_cli --help
```
You can also get detailed information about the parameters of a specific tool by running, for example:
```
czkawka_cli dup --help
```


Example usage:
```shell
czkawka dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hash -f results.txt -D aeo
czkawka empty-folders -d /home/rafal/rr /home/gateway -f results.txt
czkawka big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt
czkawka empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt
czkawka temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D
czkawka music -d /home/rafal -e /home/rafal/Pulpit -z "artist,year, ARTISTALBUM, ALBUM___tiTlE"  -f results.txt
czkawka symlinks -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt
```

## LICENSE

MIT