# Krokiet

Krokiet is a new Czkawka frontend written in Slint.

![Krokiet](https://github.com/user-attachments/assets/720e98c3-598a-41aa-a04b-0c0c1d8a28e6)

A different toolkit means a different look, limitations, and features, so you should not expect the same functionality as in the Czkawka GTK 4 frontend (but of course, I aim to implement most features from the other project).

## Usage

Krokiet should not have any special runtime requirements, it should work on almost any modern device.

Prebuilt binaries are available for Windows 10/11, Mac, and Ubuntu 22.04 (and similar distros with the same or newer glibc version): https://github.com/qarmin/czkawka/releases/

## Compilation

On Ubuntu, you need to install these dependencies:

```
sudo apt install libfontconfig-dev libfreetype-dev
```

The default compilation is done with `cargo build --release` and should work on most systems.

## Additional Renderers

By default, only femtovg (OpenGL) and the software renderer are enabled, but you can enable more renderers by compiling the app
with additional features.

Most users will want to use the app with a windowing system/compositor, so features starting with `winit` in the name are
recommended.

For example:

```
cargo build --release --features "winit_skia_opengl"
cargo build --release --features "winit_software"
```

To run the app with a different renderer, set the `SLINT_BACKEND` environment variable(but app must be compiled with the appropriate feature):

```
SLINT_BACKEND=winit-femtovg ./target/release/krokiet
SLINT_BACKEND=software ./target/release/krokiet
SLINT_BACKEND=skia ./target/release/krokiet
```

If you use an invalid or non-existing backend, the app will show a warning:

```
slint winit: unrecognized renderer skia, falling back to FemtoVG
```

To check which backend is actually used, add the `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay` environment variable:

```
SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay cargo run
```

You should see output like:

```
Slint: Build config: debug; Backend: software
```

## Scaling the Application

By default, the Slint application will automatically scale to match your system settings, but you can also manually set the scaling factor with the `SLINT_SCALE_FACTOR` environment variable:

```
SLINT_SCALE_FACTOR=2 cargo run 
```

## Different Theme

By default, Czkawka was created with the `fluent` theme in mind, but Slint also supports other themes, which are not officially supported by this app and may look broken.

```
SLINT_STYLE=cupertino-light cargo run -- --path .
SLINT_STYLE=cupertino-dark cargo run -- --path .
SLINT_STYLE=material-light cargo run -- --path .
SLINT_STYLE=material-dark cargo run -- --path .
```

## Why create a new frontend instead of improving the existing Czkawka GTK 4 app?

For many, it might seem surprising to abandon the existing GTK 4 frontend of Czkawka especially considering that GTK is one of the most popular GUI frameworks and replace it with a new one based on Slint, which is still relatively unknown.

This decision was driven by several key factors:
- **GTK on Windows and macOS performs poorly** – There are random bugs that don’t appear on Linux or on other systems with similar environments. Slint, on the other hand, behaves consistently and reliably across all platforms.
- **Complicated compilation and cross-compilation** – Due to GTK’s complexity on Windows, the easiest way to compile the application is by using a Docker image with Linux. This makes testing and debugging on Windows much more difficult.
- **External dependencies** – I’m a fan of applications that work right after downloading, without requiring installation. With GTK, this is rarely the case. On Linux and macOS, several dynamically linked libraries must be installed first, and they may exist in different versions across systems. On Windows, you often have to manually include DLLs. This wouldn't be such an issue if the GTK team officially distributed these libraries and maintained a list of required files, but they don’t, so you’re left compiling everything yourself or, like in my case, relying on external Docker images. With Slint, all I need is a single binary file that runs out of the box on almost any system.
- **GTK version fragmentation across platforms** – On Linux, GTK is dynamically linked, and different versions may introduce unique bugs or inconsistencies. On Windows, the libraries are bundled, but outdated in my app, since newer ones aren’t available in the Docker image I use, and some versions crash on some OSes. macOS (with Homebrew) is in the best position here, as it usually keeps GTK up to date. With Slint, each Krokiet release is bundled with the latest Slint version, ensuring consistency across all systems and reducing platform-specific issues.
- **Cambalache is the only no-code GUI tool** – While Cambalache itself works reasonably well, it isn’t officially supported or maintained by the GTK team, but by an independent developer. In contrast, while the Slint GUI is mostly created via code, it offers live previews in VS Code/VSCodium, which is extremely convenient.
- **Difficult to modify built-in widgets** – GTK enforces a specific visual style, which can be very restrictive. In my case, I had to tweak internal widget parameters just to achieve the desired look, something that might cause a lot of issues in the future. Slint takes the opposite approach: its built-in widgets are quite limited, which often makes it easier to build fully custom components from scratch.
- **GTK is still C code** – Even though the library is wrapped and provides a relatively safe Rust interface, you still occasionally have to work with low-level structures, which have caused issues and crashes for me in the past. Another downside is the large number of warnings printed to the console, even with correct code, due to internal GTK issues. These warnings are often unhelpful and rarely assist in identifying actual bugs.

## License

The code is licensed under the MIT license, but the entire project is licensed under GPL-3.0 due to Slint license restrictions.

All icons and images are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.

## Name

Why Krokiet (eng. Croquette)?  
Because I like croquettes (the Polish version), the ones with meat and mushrooms wrapped in breadcrumbs... it makes my mouth water.  
I also considered other dishes I like, such as pierogi, żurek, pączek, schabowy, or zapiekanka.  
This name should be much easier to remember than czkawka or szyszka.
