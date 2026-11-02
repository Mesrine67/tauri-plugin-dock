use serde::{Deserialize, Serialize};

/// Configuration compatible with HKCU\\Software\\dock (legacy migration).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockSettings {
    pub folder: String,
    pub toggle_mode: bool,
    pub show_folder_name: bool,
    pub show_item_count: bool,
    pub backdrop: String,
    pub opacity: u32,
    pub rounded_corners: bool,
    pub animations: bool,
    pub grid_columns: u32,
    pub grid_rows: u32,
    pub padding: u32,
    pub center_cursor: bool,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for DockSettings {
    fn default() -> Self {
        Self {
            folder: String::new(), toggle_mode: false,
            show_folder_name: false, show_item_count: false,
            backdrop: "acrylic".into(), opacity: 45,
            rounded_corners: true, animations: true,
            grid_columns: 0, grid_rows: 0, padding: 20,
            center_cursor: true, offset_x: 0, offset_y: 0,
        }
    }
}

impl DockSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.folder.contains('\0') || self.folder.encode_utf16().count() > 1023 {
            return Err("Invalid folder path".into());
        }
        if !self.folder.is_empty() && !std::path::Path::new(&self.folder).is_dir() {
            return Err("The selected folder does not exist".into());
        }
        if !(10..=100).contains(&self.opacity) { return Err("Opacity must be 10–100".into()); }
        if self.grid_columns > 10 || self.grid_rows > 10 || self.padding > 60 {
            return Err("Invalid grid or padding".into());
        }
        if !(-1000..=1000).contains(&self.offset_x) ||
           !(-1000..=1000).contains(&self.offset_y) {
            return Err("Invalid position offset".into());
        }
        if self.backdrop != "acrylic" && self.backdrop != "solid" {
            return Err("Invalid backdrop style".into());
        }
        Ok(())
    }
}

#[cfg(windows)]
pub fn load() -> Result<DockSettings, String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Software\\dock").ok();
    let value = |name: &str, fallback: u32| -> u32 {
        key.as_ref().and_then(|k| k.get_value::<u32, _>(name).ok()).unwrap_or(fallback)
    };
    let boolean = |name: &str, fallback: bool| value(name, u32::from(fallback)) != 0;
    let base = DockSettings::default();
    Ok(DockSettings {
        folder: key.as_ref().and_then(|k| k.get_value::<String, _>("Folder").ok()).unwrap_or_default(),
        toggle_mode: boolean("ToggleMode", base.toggle_mode),
        show_folder_name: boolean("ShowFolderName", base.show_folder_name),
        show_item_count: boolean("ShowItemCount", base.show_item_count),
        backdrop: if value("Backdrop", 1) == 0 { "solid" } else { "acrylic" }.into(),
        opacity: value("Opacity", base.opacity).clamp(10, 100),
        rounded_corners: boolean("RoundedCorners", base.rounded_corners),
        animations: boolean("Animations", base.animations),
        grid_columns: value("GridColumns", base.grid_columns).min(10),
        grid_rows: value("GridRows", base.grid_rows).min(10),
        padding: value("Padding", base.padding).min(60),
        center_cursor: boolean("CenterCursor", base.center_cursor),
        offset_x: (value("OffsetX", 0) as i32).clamp(-1000, 1000),
        offset_y: (value("OffsetY", 0) as i32).clamp(-1000, 1000),
    })
}

#[cfg(windows)]
pub fn save(config: DockSettings) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    config.validate()?;
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\dock").map_err(|e| e.to_string())?;
    let write = |name: &str, v: u32| key.set_value(name, &v).map_err(|e| e.to_string());
    key.set_value("Folder", &config.folder).map_err(|e| e.to_string())?;
    write("ToggleMode", config.toggle_mode.into())?;
    write("ShowFolderName", config.show_folder_name.into())?;
    write("ShowItemCount", config.show_item_count.into())?;
    write("Backdrop", u32::from(config.backdrop == "acrylic"))?;
    write("Opacity", config.opacity)?;
    write("RoundedCorners", config.rounded_corners.into())?;
    write("Animations", config.animations.into())?;
    write("GridColumns", config.grid_columns)?;
    write("GridRows", config.grid_rows)?;
    write("Padding", config.padding)?;
    write("CenterCursor", config.center_cursor.into())?;
    write("OffsetX", config.offset_x as u32)?;
    write("OffsetY", config.offset_y as u32)?;
    crate::native::reload()
}

#[cfg(test)]
mod tests {
    use super::DockSettings;
    #[test]
    fn validate_rejects_invalid_values() {
        let mut config = DockSettings::default();
        assert!(config.validate().is_ok());
        config.opacity = 9;
        assert!(config.validate().is_err());
        config.opacity = 45;
        config.grid_columns = 11;
        assert!(config.validate().is_err());
        config.grid_columns = 0;
        config.backdrop = "unsupported".into();
        assert!(config.validate().is_err());
    }
}
