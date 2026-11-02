//! Win32 bridge. No native handles are leaked into JavaScript.
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

unsafe extern "C" {
    fn dock_native_start() -> bool;
    fn dock_native_show() -> bool;
    fn dock_native_hide() -> bool;
    fn dock_native_toggle() -> bool;
    fn dock_native_open_settings() -> bool;
    fn dock_native_stop() -> bool;
    fn dock_native_is_visible() -> bool;
    fn dock_native_set_monitor(monitor: *const u16) -> bool;
    fn dock_native_reload_settings() -> bool;
}

fn result(ok: bool, operation: &str) -> Result<(), String> {
    if ok { Ok(()) } else { Err(format!("Native Dock failed to {operation}")) }
}

pub fn show() -> Result<(), String> { result(unsafe { dock_native_show() }, "show") }
pub fn hide() -> Result<(), String> { result(unsafe { dock_native_hide() }, "hide") }
pub fn toggle() -> Result<(), String> { result(unsafe { dock_native_toggle() }, "toggle") }
pub fn open_settings() -> Result<(), String> {
    result(unsafe { dock_native_open_settings() }, "open settings")
}
pub fn close() -> Result<(), String> { result(unsafe { dock_native_stop() }, "stop") }
pub fn is_visible() -> bool { unsafe { dock_native_is_visible() } }
pub fn reload() -> Result<(), String> {
    result(unsafe { dock_native_reload_settings() }, "reload settings")
}
pub fn set_monitor(value: &str) -> Result<(), String> {
    if value.len() > 256 || value.contains('\0') {
        return Err("Invalid monitor identifier".into());
    }
    let wide: Vec<u16> = OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect();
    result(unsafe { dock_native_set_monitor(wide.as_ptr()) }, "select monitor")
}
#[allow(dead_code)]
pub fn start() -> Result<(), String> {
    result(unsafe { dock_native_start() }, "start")
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_library_links_without_starting_a_window() {
        // This forces MSVC to link the C++ engine into a real executable.
        let _ = super::is_visible();
    }
}
