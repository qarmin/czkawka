# Krokiet

Krokiet is new Czkawka frontend written in Slint(written mostly in Rust) in opposite to Gtk 4 frontend which uses mostly
C code.

Different toolkit means different look, limitations and features, so you should not expect same features like in Gtk 4
frontend(but of course I want implement most of features from other project).

## Requirements

Krokiet should not have any special runtime requirements - it should work on almost any OpenGL ES 2 capable device.

Alternatively, it can be run with software rendering on computers that don't have graphics card.

## Compilation

On Ubuntu you need to install this dependencies:

```
sudo apt install libfontconfig-dev libfreetype-dev
```

Default compilation is done by `cargo build --release` and should work on most systems.

You need the latest available version of Rust to compile it, because Krokiet aims to support the latest slint verions,
that should provide best experience(fixed more bugs/new features).

The only exception is building non default skia renderer, that require on windows msvc compiler(not sure how to exactly
install it).

Also skia renderer is written in C++ and uses on platforms like x86_64 and arm64 prebuild binaries, so if you are using
different architecture, this library will be build from source, which can take a lot of time and require additional
dependencies.

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

## Dark/Different theme

App was created with white fluent theme by default, but is easily possible to use dark theme by setting `SLINT_STYLE`
environment variable to `fluent-dark` during compilation
e.g.

```
SLINT_STYLE=fluent-dark cargo run -- --path .
```

Slint supports also other themes, but they are not officially supported by this app and may be broken.

```
SLINT_STYLE=cupertino-light cargo run -- --path .
SLINT_STYLE=cupertino-dark cargo run -- --path .
SLINT_STYLE=material-light cargo run -- --path .
SLINT_STYLE=material-dark cargo run -- --path .
```

## How to help?

- Suggesting possible design changes in the gui - of course, they should be possible to be simply implemented in the
  slint keeping in mind the performance aspect as well
- Modifying user interface - gui is written in simple language similar to qml, that can be modified in vscode/web with
  live preview - [slint live preview example](https://slint.dev/releases/1.2.2/editor/?load_demo=examples/printerdemo/ui/printerdemo.slint)
- Improving libraries used by Krokiet e.g. czkawka_core, image-rs etc.
- Improving app rust code

## Missing features available in GTK 4 frontend

- preview support
- selecting/deselecting results
- icons in buttons
- resizable input files panel
- settings
- moving files
- deleting files
- implementing all modes
- multiple languages

## Why Slint?

There are multiple reasons why I decided to use Slint as toolkit for Krokiet over other toolkits.

| Toolkit | Pros                                                                                                                                                                                                                    | Cons                                                                                                                                                                                                                                                                                                                                                                                            |
|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Gtk 4   | - Hard compilation/cross compilation and bundling all required libraries - mostly on windows </br> - Cambalache can be used to create graphically gui </br> - Good gtk4-rs bindings(but sometimes not really intuitive) | - Hard compilation/cross compilation and bundling all required libraries - mostly on windows </br> - Forcing the use of a specific gui creation style </br> - Strange crashes, not working basic features, etc.(again, mostly on windows) </br> - Forcing to use bugged/outdated but dynamically loaded version of libraries on linux (e.g. 4.6 on Ubuntu 22.04) - not all fixes are backported |
| Qt      | - QML support - simplify creating of gui from code it is easy to use and powerful </br> - Very flexible framework <br/> - Typescript/javascript <=> qml interoperability </br> - Probably the most mature GUI library   | - New and limited qt bindings <br/> - Big cross compilation problems and hard to setup on non windows platforms <br/>  - Very easy to create and use invalid state in QML(unexpected null/undefined values etc.) <br/> - Commercial license or GPL                                                                                                                                              |
| Slint   | - Internal language is compiled to native code <br/> - Live gui preview with Vscode/Vscodium without needing to use rust <br/> - Full rust solution - easy to compile/cross compile, minimal runtime requirements       | - Internal .slint language is more limited than QML(javascript flexibility is big pl) <br/> - Out of bounds and similar errors are quietly being corrected instead printing error - this can lead to hard to debug problems    <br/> - Commercial license or GPL(is available also different )                                                                                                  |
| Iced    | - ~100% rust code - so compilation is simple </br> - Elm architecture - simple to understand                                                                                                                            | - Mostly maintained by one person - slows down fixing bugs and implementing new features </br> - GUI can be created only from rust code, which really is bad for creating complex GUIs(mostly due rust compile times) and this is also  </br> - Docs are almost non-existent                                                                                                                    |
| Tauri   | - Easy to create ui(at least for web developers) - uses html/css/js</br>- Quite portable                                                                                                                                | - Webview dependency - it is not really lightweight and can be hard to compile on some platforms</br>- Cannot select directory - file chooser only can choose files - small thing but important for me</br>- Not very performant Rust <=> Javascript communication                                                                                                                              |

Since I don't have time to create really complex and good looking GUI, I needed a helper tool to create GUI not from
Rust(I don't want to use different language, because this will make communication with czkawka_core harder) so I decided
to not look at Iced which only allows to create GUI from Rust.

GTK and QT also I throw away due cross compilation problems caused mostly by using C/C++ internally.

Tauri - I don't really like to use Javascript because I already used it with Qt(C++) + QML + Typescript combination and
I found that creating ui in such language may be simple at start but later any bigger changes cause a lot of runtime
errors.

So only Slint left with its cons and pros.

## Name

Why Krokiet(eng. Croquette)?  
Because I like croquettes(Polish version), the ones with meat, mushrooms wrapped in breadcrumbs... it makes my mouth
water.  
I considered also other dishes which I like to eat like pierogi, żurek, pączek, schabowy or zapiekanka.  
This name should be a lot of easier to remember than Czkawka.