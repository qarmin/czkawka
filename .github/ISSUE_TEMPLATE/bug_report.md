---
name: Bug report
about: Create a report to help us improve app
title: ''
labels: bug
assignees: ''

---

**Bug Description**

**Steps to reproduce:**
<!-- Please describe what you expected to see and what you saw instead. Also include screenshots or screencasts if needed. -->

**Terminal output** (optional):

```
<!--
Add terminal output only if needed - if there are some errors or warnings or you have performance/freeze issues.  
Very helpful in this situation will be logs from czkawka run with RUST_LOG environment variable set e.g. 
`RUST_LOG=debug ./czkawka` or `flatpak run --env=RUST_LOG=debug com.github.qarmin.czkawka` if you use flatpak, which will print more detailed info about executed function.
-->

<details>
<summary>Debug log</summary>

# UNCOMMENT DETAILS AND PUT LOGS HERE

</details>
```

**System**

<!-- OS and Czkawka/Krokiet version and other OS info - you can copy it from the logs if you run the app from a terminal or locate the log files manually
(Linux: `/home/username/.cache/czkawka`,
macOS: `/Users/Username/Library/Caches/pl.Qarmin.Czkawka`,
Windows: `C:\Users\Username\AppData\Local\Qarmin\Czkawka\cache`).
Note: the exact path depends on the installation method(you can open config/cache path from gui). -->
<!-- Example of logs: -->
<!-- Czkawka gtk version: 11.0.0, debug mode, rust 1.92.0 (2025-06-23), os Ubuntu 25.4.0 (x86_64 64-bit), 24 cpu/threads, features(1): [fast_image_resize], app cpu version: x86-64-v3 (AVX2) or x86-64-v4 (AVX-512), os cpu version: x86-64-v4 (AVX-512) -->
<!-- Config folder set to "/home/rafal/.config/czkawka" and cache folder set to "/home/rafal/.cache/czkawka" -->
<!-- Czkawka Gui - used thread number: 24, gtk version 4.18.5 -->

<!-- Please do not report feature request unique for Gtk Czkawka gui, because it is in maintenance mode. -->

- Czkawka/Krokiet version: <!--  e.g. 11.0.0 cli/gui -->
- OS version: <!--  e.g. Ubuntu 22.04, Windows 11, Mac 15.1 ARM -->
- Installation method: <!-- e.g. github binaries, flatpak, msys2 -->

<!-- If you use flatpak, please include the result of `flatpak info com.github.qarmin.czkawka`. -->
