# Migration of FXMP OS Dock into a Tauri 2 plugin

## Current behavior

FXMP OS currently bundles a separate native `dock.exe` and controls it with `--fxmp-*` command line arguments. The integrated fork additionally supports monitor selection and suppresses duplicate tray/hotkey behavior.

The new plugin must be tested as an in-process replacement before removing the old executable.

## Native SettingsWindow.cpp field parity

- Folder chooser, Explorer display names, asynchronously loaded icons and file change refresh
- Shortcut capture and hold/toggle mode
- Selected monitor, cursor centering, position x/y offsets
- Grid columns, rows, padding, header name and item count
- Acrylic/solid background, opacity, rounded corners, animations
- Live visual preview and scrolling
- Autostart and updater controls must move to the Tauri host

## Merge gates

1. CI Rust and C++ compilation on MSVC, and TypeScript compilation.
2. Repeated start, show, hide, toggle, stop cycles without deadlock.
3. Accurate input, cursor and focus behavior in front of running applications.
4. Multiple displays at distinct DPI scales and disconnected-monitor fallback.
5. Folder watcher and icon updates without stale worker crashes.
6. Equivalent TSX Settings panel with persistence and live preview.
7. FXMP OS PR and Windows installer/updater verification.
8. Remove original executable, CMake and resource declaration only when all gates are satisfied.

## Git branch tracking

FXMP OS release/v0.0.44-tray-polish diverges (ahead 3, behind 15). release/v0.0.48 diverges (ahead 2, behind 11) and contains generated CMake artifacts. Do not merge either wholesale into main. Security issue #7 (vulnerable Linux GLib) is independent and remains open.
