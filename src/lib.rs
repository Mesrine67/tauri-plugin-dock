mod commands;
#[cfg(windows)]
mod native;
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("dock")
        .invoke_handler(tauri::generate_handler![
            commands::show, commands::hide, commands::toggle,
            commands::close, commands::open_settings,
            commands::set_monitor, commands::get_status,
        ])
        .build()
}
