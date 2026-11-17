
![krokiet_logo](https://github.com/user-attachments/assets/f5e4b290-d001-4cf4-9f52-dab65a30e441)

Krokiet is a new Czkawka frontend written in Slint.

![Krokiet](https://github.com/user-attachments/assets/720e98c3-598a-41aa-a04b-0c0c1d8a28e6)

It aims to provide a more consistent experience across all platforms (Linux, Windows, macOS).

## How to install?
Prebuilt binaries are available for Windows 10/11, Mac and Linux with glibc 2.17 or newer (x86_64 and arm64) - almost every Linux distribution released after 2014 is supported.

You can download them from https://github.com/qarmin/czkawka/releases/, which contains recommendations, which variant to use depending on your needs.

## Requirements

Prebuilt binaries have no mandatory runtime dependencies.

The similar videos tool requires **ffmpeg** at runtime.

### Linux (Ubuntu / Debian)

```shell
# Runtime: similar videos
sudo apt install ffmpeg
```

### macOS

```shell
brew install ffmpeg
```

### Windows

- ffmpeg: `choco install ffmpeg` or download from [ffmpeg.org](https://ffmpeg.org/download.html#build-windows) and place `ffmpeg.exe` in your `PATH`.

## Compilation

Another option is to compile it yourself.

The easiest way is to compile newest Krokiet, is to run
```
cargo install krokiet --locked
```

which will install, the latest and optimized version of Krokiet.

Compilation with `cargo build --release` should produce a working binary, that without any additional dependencies should run on user os.

If you have installed new `cargo`, you can easily compile and install it via `cargo install krokiet`

## Additional Renderers

By default, only femtovg (OpenGL) and the software renderer are enabled, but you can enable more renderers by compiling the app
with additional features (`femtovg_wgpu`, `vello`). Skia is not supported.

When `SLINT_BACKEND` is not set, the first renderer that starts successfully is used, in this order: femtovg (OpenGL), femtovg-wgpu, vello, software.

Most users will want to use the app with a windowing system/compositor, so features starting with `winit` in the name are
recommended.

For example:

```
cargo build --release --features "femtovg_wgpu"
cargo build --release --features "vello"
cargo build --release --features "winit_software"
```

To run the app with a different renderer, set the `SLINT_BACKEND` environment variable(but app must be compiled with the appropriate feature):

```
SLINT_BACKEND=winit-femtovg ./target/release/krokiet
SLINT_BACKEND=software ./target/release/krokiet
SLINT_BACKEND=femtovg-wgpu ./target/release/krokiet
SLINT_BACKEND=vello ./target/release/krokiet
```

If you use an invalid or non-existing backend, the app will show a warning:

```
slint winit: unrecognized renderer vello, falling back to FemtoVG
```

To check which backend is actually used, add the `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay` environment variable:

```
SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay cargo run
```

You should see output like:

```
Slint: Build config: debug; Backend: software
```


## License

The code is licensed under the MIT license, but the entire project is licensed under GPL-3.0 due to Slint license restrictions.

All icons and images are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.

## Name

Why Krokiet (eng. Croquette)?  
Because I like croquettes (the Polish version), the ones with meat and mushrooms wrapped in breadcrumbs... it makes my mouth water.  
I also considered other dishes I like, such as pierogi, żurek, pączek, schabowy, or zapiekanka.  
This name should be much easier to remember than czkawka or szyszka.
