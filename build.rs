const COMMANDS: &[&str] = &[
    "show", "hide", "toggle", "close", "open_settings", "set_monitor", "get_status", "get_settings", "set_settings"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();

    #[cfg(target_os = "windows")]
    {
        let mut build = cc::Build::new();
        build.cpp(true)
            .include("native/dock/src")
            .std("c++17")
            .define("UNICODE", None)
            .define("_UNICODE", None)
            .define("WIN32_LEAN_AND_MEAN", None)
            .define("NOMINMAX", None)
            .define("DOCK_VERSION", Some("\"0.1.0\""))
            .flag_if_supported("/EHsc")
            .flag_if_supported("/utf-8");

        for name in [
            "main.cpp", "App.cpp", "FolderModel.cpp", "FolderWatcher.cpp",
            "KeyboardHook.cpp", "OverlayWindow.cpp", "Settings.cpp",
            "SettingsWindow.cpp", "UpdateChecker.cpp",
        ] {
            build.file(format!("native/dock/src/{name}"));
        }
        build.file("src/native_bridge.cpp");
        build.compile("fxmp_dock_native");

        for name in [
            "d2d1", "dwrite", "windowscodecs", "shell32", "shlwapi", "ole32",
            "user32", "gdi32", "dwmapi", "uxtheme", "comctl32", "shcore",
            "winhttp", "bcrypt",
        ] {
            println!("cargo:rustc-link-lib={name}");
        }
    }
}
