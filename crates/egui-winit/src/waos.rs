//! WaOS: one build of an app runs on every system WaOS runs on, so which system it is on is
//! asked of WaOS as the app starts, where every other target reads it from the build. What the
//! system's bars and the keyboard cover of the app's view is asked of WaOS too, as egui-winit asks
//! UIKit on iOS: on a phone they are drawn over the view.

use egui::os::OperatingSystem;
use egui::{SafeAreaInsets, epaint::MarginF32};
use raw_window_handle::HasWindowHandle as _;
use wasdk::system;
use winit::window::Window;

/// The system WaOS runs the app on, as egui names it.
pub(crate) fn operating_system() -> OperatingSystem {
    match system::os() {
        system::OperatingSystem::Android => OperatingSystem::Android,
        system::OperatingSystem::Ios => OperatingSystem::IOS,
        system::OperatingSystem::Macos => OperatingSystem::Mac,
        system::OperatingSystem::Windows => OperatingSystem::Windows,
        system::OperatingSystem::Linux => OperatingSystem::Nix,
        system::OperatingSystem::Fuchsia | system::OperatingSystem::Other => {
            OperatingSystem::Unknown
        }
    }
}

/// What the system's bars and the keyboard cover of `window`'s view now, in points, which egui
/// keeps its content clear of: nothing on a desktop. None for a window that is not a WaOS view.
pub(crate) fn safe_area_insets(window: &Window) -> Option<SafeAreaInsets> {
    let view = wasdk::view_of_window_handle(window.window_handle().ok()?)?;
    let metrics = view.metrics();
    let pixels_per_point = metrics.device_pixel_ratio as f32;
    let (bars, keyboard) = (metrics.padding, metrics.view_insets);
    let covered = |bar: f64, keys: f64| (bar + keys) as f32 / pixels_per_point;
    Some(SafeAreaInsets(MarginF32 {
        left: covered(bars.left, keyboard.left),
        right: covered(bars.right, keyboard.right),
        top: covered(bars.top, keyboard.top),
        bottom: covered(bars.bottom, keyboard.bottom),
    }))
}
