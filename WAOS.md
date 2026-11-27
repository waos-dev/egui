# egui for WaOS

This fork makes egui 0.36.2 right on a WaOS app (a WASI component built for `wasm32-wasip3`, `target_os = "wasi"`). Upstream reads every `wasm32` target as a browser; a WaOS app is not one. The changes are in `crates/egui` and `crates/egui-winit`; everything else is upstream's. It is kept for whoever ports an egui app to WaOS, and is not offered upstream.

## Using it

egui is one repository, so an app takes every egui crate it uses from the fork together:

```toml
[patch.crates-io]
egui = { git = "https://github.com/waos-dev/egui", branch = "waos" }
emath = { git = "https://github.com/waos-dev/egui", branch = "waos" }
ecolor = { git = "https://github.com/waos-dev/egui", branch = "waos" }
epaint = { git = "https://github.com/waos-dev/egui", branch = "waos" }
epaint_default_fonts = { git = "https://github.com/waos-dev/egui", branch = "waos" }
egui-winit = { git = "https://github.com/waos-dev/egui", branch = "waos" }
egui-wgpu = { git = "https://github.com/waos-dev/egui", branch = "waos" }
egui_extras = { git = "https://github.com/waos-dev/egui", branch = "waos" }
```

`egui-winit` goes with WaOS's winit (`waos-dev/winit`), and its `clipboard` feature with WaOS's arboard (`waos-dev/arboard`). `egui-winit` reads what the system's bars and the keyboard cover of the app's view, as it does on iOS, so egui keeps its content clear of them on a phone, through the published `wasdk` (0.8.0).

## What changes on WaOS

- **A dropped file is read as on a desktop.** `DroppedFile` has `bytes`, the read at once from its path, and not the browser's `bytes_async` and `web_file`. This is also what lets `egui-winit` build, which upstream does not for any `wasm32` target.
- **No browser libraries.** `egui` no longer asks for `web-sys` on WaOS.
- **The scroll speed of a wheel's lines** is a desktop's, not a browser's.
- **The system is asked of WaOS.** One build runs on every system WaOS runs on, so `egui-winit`'s `State::new` sets the context's system (`Context::set_os`) from WaOS's answer, and the shortcut key follows it: Command on a Mac, Control elsewhere. Upstream reads both from the build.

## What stays as on any wasm32 target

- `egui-wgpu` and `egui_extras` are untouched. Where they switch on `wasm32` it is for what holds on WaOS too: wgpu's types are not shared between threads, adapters are not listed, and images are decoded without a thread of their own.
