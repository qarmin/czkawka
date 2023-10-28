# NAME_TODO

NAME_TODO is new Czkawka frontend written in Slint(written mostly in Rust) in opposite to Gtk 4 frontend which uses mostly C code.

Different  

## Requirements
Slint version of app should not have any special runtime requirements - it should work on almost any OpenGL 3 capable device.

Alternatively, it can be run with software rendering.

## Compilation
Default compilation is done by `cargo build --release`.

All dependencies should be handled by cargo, except windows where you need to install m.

The only exception is building skia frontend, that require 



