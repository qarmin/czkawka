# Krokiet

Krokiet is new Czkawka frontend written in Slint.

![Krokiet](https://github.com/user-attachments/assets/720e98c3-598a-41aa-a04b-0c0c1d8a28e6)

Different toolkit means different look, limitations and features, so you should not expect same features like in Czkawka
Gtk 4 frontend(but of course I want implement most of the features from other project).

## Usage

Krokiet should not have any special runtime requirements - it should work on almost any device non-antic device.

Prebuild binaries should work on Windows 10,11, Mac, Ubuntu 22.04(and similar distros, with same or greater glibc version) - https://github.com/qarmin/czkawka/releases/

## Compilation

On Ubuntu you need to install this dependencies:

```
sudo apt install libfontconfig-dev libfreetype-dev
```

Default compilation is done by `cargo build --release` and should work on most systems.

## Additional Renderers

By default, only femtovg(opengl) and software renderer are enabled, but you can enable more renderers by compiling app
with additional features.

Most of the users will want to use app with windowing system/compositor, so features starting with `winit` in name are
recommended.

E.g.

```
cargo build --release --features "winit_skia_opengl"
cargo build --release --features "winit_software"
```

to run app with different renderers you need to use it, by adding `SLINT_BACKEND` environment

```
SLINT_BACKEND=winit-femtovg ./target/release/krokiet
SLINT_BACKEND=software ./target/release/krokiet
SLINT_BACKEND=skia ./target/release/krokiet # This uses now opengl - https://github.com/slint-ui/slint/discussions/3799
```

when you will use invalid/non-existing backend, app will show warning

```
slint winit: unrecognized renderer skia, falling back to FemtoVG
```

to check what is really used, add `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay` env

```
SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay cargo run
```

should print something like

```
Slint: Build config: debug; Backend: software
```

## Scaling application

By default, slint application will automatically scale, to match your system scaling, but it is also possible to manually set scalling factor with `SLINT_SCALE_FACTOR` environment

```
SLINT_SCALE_FACTOR=2 cargo run 
```

## Different theme
By default, Czkawka was created with `fluent` theme in mind but slint supports also other themes which are not officially supported by this app and may looks broken.

```
SLINT_STYLE=cupertino-light cargo run -- --path .
SLINT_STYLE=cupertino-dark cargo run -- --path .
SLINT_STYLE=material-light cargo run -- --path .
SLINT_STYLE=material-dark cargo run -- --path .
```

## Why create app with a new frontend instead of improving the existing Czkawka GTK 4 app?

For many, it might seem like a surprising decision to abandon the existing GTK 4 frontend of Czkawka—especially considering that GTK is one of the most popular GUI frameworks and replace it with a new one based on Slint, which is still relatively unknown.

This decision was driven by several key factors:
- **GTK on Windows and macOS performs poorly** – There are random bugs that don’t appear on Linux, nor on other systems with similar environments. Slint, on the other hand, behaves consistently and reliably across all platforms.

- **Complicated compilation and cross-compilation** – Due to GTK’s complexity on Windows, the easiest way to compile the application is by using a Docker image with Linux. This makes testing and debugging on Windows much more difficult.

- **External dependencies** – I’m a fan of applications that work right after downloading, without requiring installation. With GTK, this is rarely the case. On Linux and macOS, several dynamically linked libraries must be installed first and they may exist in different versions across systems. On Windows, you often have to manually include DLLs. This wouldn't be such an issue if the GTK team officially distributed these libraries and maintained a list of required files, but they don’t, so you’re left compiling everything yourself or, like in my case, relying on external Docker images. With Slint, all I need is a single binary file that runs out of the box on almost any system.

- **GTK version fragmentation across platforms** – On Linux, GTK is dynamically linked, and different versions may introduce unique bugs or inconsistencies. On Windows, the libraries are bundled, but outdated in my app, since newer ones aren’t available on docker image which I use and some versions crashes on some os. macOS (with Homebrew) is in the best position here, as it usually keeps GTK up to date. With Slint, each Krokiet release is bundled with the latest Slint version, ensuring consistency across all systems and reducing platform-specific issues.

- **Cambalache is the only nocode GUI tool** – While Cambalache itself works reasonably well, it isn’t officially supported or maintained by the GTK team, but by an independent developer. In contrast, while Slint GUI is mostly created via code, it offers live previews in VS Code/VSCodium, which is extremely convenient.

- **Difficult to modify built-in widgets** – GTK enforces a specific visual style, which can be very restrictive. In my case, I had to tweak internal widget parameters just to achieve the desired look, this is something that might cause a lot of issues in future. Slint takes the opposite approach: its built-in widgets are quite limited, which often makes it easier to build fully custom components from scratch.

- **GTK is still C code** – Even though the library is wrapped and provides a relatively safe Rust interface, you still occasionally have to work with low-level structures, which have caused issues and crashes for me in the past. Another downside is the large number of warnings printed to the console, even with correct code, due to internal GTK issues. These warnings are often unhelpful and rarely assist in identifying actual bugs.

## License

Code is licensed under MIT license but entire project is licensed under GPL-3.0 license, due Slint license restrictions.
Icons

## Name

Why Krokiet(eng. Croquette)?  
Because I like croquettes(Polish version), the ones with meat, mushrooms wrapped in breadcrumbs... it makes my mouth
water.  
I considered also other dishes which I like to eat like pierogi, żurek, pączek, schabowy or zapiekanka.  
This name should be a lot of easier to remember than czkawka or szyszka.
