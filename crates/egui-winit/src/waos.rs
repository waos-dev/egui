//! WaOS: one build of an app runs on every system WaOS runs on, so which system it is on is
//! asked of WaOS as the app starts, where every other target reads it from the build.

use egui::os::OperatingSystem;
use wasdk::system;

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
