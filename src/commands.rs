use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockStatus {
    pub visible: bool,
    pub supported: bool,
}

#[cfg(not(windows))]
fn unavailable() -> Result<(), String> {
    Err("Dock currently supports Windows 10/11 only".to_string())
}

#[tauri::command]
pub fn show() -> Result<(), String> {
    #[cfg(windows)] { return crate::native::show(); }
    #[cfg(not(windows))] { unavailable() }
}
#[tauri::command]
pub fn hide() -> Result<(), String> {
    #[cfg(windows)] { return crate::native::hide(); }
    #[cfg(not(windows))] { unavailable() }
}
#[tauri::command]
pub fn toggle() -> Result<(), String> {
    #[cfg(windows)] { return crate::native::toggle(); }
    #[cfg(not(windows))] { unavailable() }
}
#[tauri::command]
pub fn close() -> Result<(), String> {
    #[cfg(windows)] { return crate::native::close(); }
    #[cfg(not(windows))] { unavailable() }
}
#[tauri::command]
pub fn open_settings() -> Result<(), String> {
    #[cfg(windows)] { return crate::native::open_settings(); }
    #[cfg(not(windows))] { unavailable() }
}
#[tauri::command]
pub fn set_monitor(monitor_id: String) -> Result<(), String> {
    #[cfg(windows)] { return crate::native::set_monitor(&monitor_id); }
    #[cfg(not(windows))] { let _ = monitor_id; unavailable() }
}
#[tauri::command]
pub fn get_status() -> DockStatus {
    DockStatus {
        visible: {
            #[cfg(windows)] { crate::native::is_visible() }
            #[cfg(not(windows))] { false }
        },
        supported: cfg!(windows),
    }
}
