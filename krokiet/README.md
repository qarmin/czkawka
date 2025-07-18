# Krokiet

Krokiet is new Czkawka frontend written in Slint.

![Krokiet](https://github.com/user-attachments/assets/531cc479-8900-4db6-aa60-598284e7f815)

Different toolkit means different look, limitations and features, so you should not expect same features like in Czkawka
Gtk 4 frontend(but of course I want implement most of the features from other project).

## Usage

Krokiet should not have any special runtime requirements - it should work on almost any device non-antic device.

Prebuild binaries should work on Windows 10,11, Mac, Ubuntu 22.04/24.04(depending on features) and similar - https://github.com/qarmin/czkawka/releases/

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

If you have high DPI monitor, you may want to scale application. You can do it by setting `SLINT_SCALE_FACTOR`
environment

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

## Sanitizer build
```
rustup install nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu

export RUST_BACKTRACE=1 # or full depending on project
export ASAN_SYMBOLIZER_PATH=$(which llvm-symbolizer-18)
export ASAN_OPTIONS="symbolize=1:detect_leaks=0" # Leak check is disabled, because is slow and ususally not needed
RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --bin krokiet --target x86_64-unknown-linux-gnu

```

## How to help?

- Suggesting possible design changes in the gui - of course, they should be possible to be simply implemented in the
  slint keeping in mind the performance aspect as well
- Modifying user interface - gui is written in simple language similar to qml, that can be modified in vscode/web with
  live
  preview - [slint live preview example](https://slint.dev/releases/1.3.0/editor/?load_demo=examples/printerdemo/ui/printerdemo.slint)
- Improving app rust code

## Why Slint?

There are multiple reasons why I decided to use Slint as toolkit for Krokiet over other toolkits.

| Toolkit | Pros                                                                                                                                                                                                                                                        | Cons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Gtk 4   | - Good Linux support </br> - Cambalache can be used to create graphically gui </br> - Good gtk4-rs bindings(but sometimes not really intuitive)                                                                                                             | - Hard compilation/cross compilation and bundling all required libraries - mostly on windows </br> - Forcing the use of a specific gui creation style </br> - Strange crashes, not working basic features, etc.(again, mostly on windows) </br> - Forcing to use bugged/outdated but dynamically loaded version of libraries on linux (e.g. 4.6 on Ubuntu 22.04) - not all fixes are backported                                                                                                                                 |
| Qt      | - QML support - simplify creating of gui from code it is easy to use and powerful </br> - Very flexible framework <br/> - Typescript/javascript <=> qml interoperability </br> - Probably the most mature GUI library                                       | - New and limited qt bindings <br/> - Hard to cross-compile <br/>  - Very easy to create and use invalid state in QML(unexpected null/undefined values, messed properties bindings etc.) <br/> - Commercial license or GPL                                                                                                                                                                                                                                                                                                      |
| Slint   | - Internal language is compiled to native code <br/> - Live gui preview with Vscode/Vscodium without needing to use rust <br/> - Full rust solution - easy to compile/cross compile, minimal runtime requirements </br> - Static type checks in slint files | - Internal .slint language is more limited than QML <br/> - Out of bounds and similar errors are quietly being corrected instead printing error - this can lead to hard to debug problems <br/> - Only GPL is only available open-source license <br/> - Popup windows almost not exists <br/> - Internal widgets are almost not customizable and usually quite limited </br> - Creating big/good looking with custom widgets gui is very hard                                                                                  |
| Iced    | - ~100% rust code - so compilation is simple </br> - Elm architecture - simple to understand                                                                                                                                                                | - Mostly maintained by one person - slows down fixing bugs and implementing new features </br> - GUI can be created only from rust code, which really is bad for creating complex GUIs(mostly due rust compile times)  </br> - Docs are almost non-existent                                                                                                                                                                                                                                                                     |
| Tauri   | - Easy to create ui(at least for web developers) - uses html/css/js</br>- Quite portable                                                                                                                                                                    | - Webview dependency - it is not really lightweight and can be hard to compile on some platforms and on Linux e.g. webRTC not working</br>- Cannot select directory - file chooser only can choose files - small thing but important for me</br>- Not very performant Rust <=> Javascript communication (less problematic with [Tauri 2](https://github.com/tauri-apps/tauri/pull/7170#issuecomment-1583279023)) </br> - Uses Javascript/Typescript which is a lot of harder to update/maintain than rust due being less strict |

Since I don't have time to create really complex and good looking GUI, I needed a helper tool to create GUI not from
Rust(I don't want to use different language, because this will make communication with czkawka_core harder) so I decided
to not look at Iced which only allows to create GUI from Rust.

GTK and QT also I throw away due cross compilation problems caused mostly by using C/C++ internally. Using GTK in
Czkawka was a reason why I started to find other toolkits.

Tauri - I don't really like to use Javascript because I already used it with Qt(C++) + QML + Typescript combination and
I found that creating ui in such language may be simple at start but later any bigger changes cause a lot of runtime
errors. Despite these problems, my next project(closed source), will just use Tauri + slint, because actually in that
case it is rather the best option.

So only Slint left with its cons and pros.

## License

Code is licensed under MIT license but entire project is licensed under GPL-3.0 license, due Slint license restrictions.
Icons

## Name

Why Krokiet(eng. Croquette)?  
Because I like croquettes(Polish version), the ones with meat, mushrooms wrapped in breadcrumbs... it makes my mouth
water.  
I considered also other dishes which I like to eat like pierogi, żurek, pączek, schabowy or zapiekanka.  
This name should be a lot of easier to remember than czkawka or szyszka.