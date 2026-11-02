//! Native Windows Dock plugin for Tauri 2.
mod commands;
mod settings;
#[cfg(windows)]
mod native;

use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

/// Rust-side API for a host app's tray menu or global shortcut callbacks.
pub mod api {
    #[cfg(windows)]
    pub use crate::native::{close, hide, open_settings, set_monitor, show, toggle};
    #[cfg(not(windows))]
    pub fn toggle() -> Result<(), String> { Err("Dock requires Windows".into()) }
    #[cfg(not(windows))]
    pub fn close() -> Result<(), String> { Ok(()) }
    #[cfg(not(windows))]
    pub fn open_settings() -> Result<(), String> { Err("Dock requires Windows".into()) }
    #[cfg(not(windows))]
    pub fn set_monitor(_monitor: &str) -> Result<(), String> { Err("Dock requires Windows".into()) }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("dock")
        .invoke_handler(tauri::generate_handler![
            commands::show, commands::hide, commands::toggle,
            commands::close, commands::open_settings,
            commands::set_monitor, commands::get_status,
            commands::get_settings, commands::set_settings,
        ])
        .on_event(|_, event| {
            #[cfg(windows)]
            if matches!(event, tauri::RunEvent::Exit) {
                let _ = crate::native::close();
            }
            #[cfg(not(windows))]
            let _ = event;
        })
        .build()
}
