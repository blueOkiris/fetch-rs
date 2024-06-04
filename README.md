# Fetch-rs

## Description

Fetch CPU, GPU, OS info, etc and display it to terminal with a nice distro-dependent graphic.

Features different from other fetch scripts:

- Single binary (assuming no plugins)
- Compiled and fast
- Asynchronous to look up information
- Plugin support

## Documentation

To change from default settings, run once to generate default configuration file then edit it: `~/.config/fetch-rs/config.json`

Supports:

- Detecting distro and drawing its logo (enabled by default via `"show_distro": true`)
   + Terminal Colors in those Logos (see `distro-logos/*.txt` for how to put them in a logo):
      * Normal Colors:
         - NBLK - "Normal" Black
         - NRED - Normal Red
         - NGRN - "" Green
         - NYLW - Yellow
         - NBLU - Blue
         - NMAG - Magenta
         - NCYN - Cyan
         - NWHT - White
      * Bright Colors
         - BBLK - "Bright" Black
         - BRED - Bright Red
         - BGRN - "" Green
         - BYLW - Yellow
         - BBLU - Blue
         - BMAG - Magenta
         - BCYN - Cyan
         - BWHT - White
- OS information (enabled by default via `"show_os": true`):
   + Distro
   + Distro Release Version
   + Distro Nickname
   + Architecture
- Host
- Kernel
- Uptime

TODO:

- Packages (def)
- Shell (def)
- Resolution (def)
- DE (def)
- WM (def)
- WM Theme (def)
- Theme (def)
- Icons (def)
- Terminal (def)
- Terminal Font (def)
- CPU (def)
- GPU (def)
- Memory (def)
- CPU Usage (not def)
- Disk (not def)
- Battery (not def)
- Font (not def)
- Song (not def)
- Local IP (not def)
- Public IP (not def)
- Users (not def)
- Birthday (not def)

You can also add custom plugins. Create a program that supports a C ABI function with this function signature: `const char *int_output(void)`

Compile a `.so` and place it in `~/.config/fetch-rs/plugins/`

It's `int_output` function will print after the built-in outputs

