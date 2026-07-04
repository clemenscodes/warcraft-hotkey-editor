use std::fmt;
use warcraft_keybinds::AbilityIconPath;

const REPLACEABLE_TEXTURES_PREFIX: &str = "replaceabletextures/";

#[derive(Clone, PartialEq, Debug)]
pub struct IconUrl {
    url: String,
}

impl IconUrl {
    fn prefix() -> String {
        match dioxus_cli_config::base_path() {
            Some(base) => {
                let trimmed = base.trim_matches('/');
                if trimmed.is_empty() {
                    String::from("/icons/")
                } else {
                    format!("/{trimmed}/icons/")
                }
            }
            None => String::from("/icons/"),
        }
    }

    pub fn from_database_path(database_icon_path: &str) -> Self {
        let lowered_path = database_icon_path.to_ascii_lowercase();
        let png_path = match lowered_path.strip_suffix(".blp") {
            Some(stem_without_extension) => format!("{stem_without_extension}.png"),
            None => lowered_path,
        };
        let prefix = Self::prefix();
        let url = format!("{prefix}{png_path}");
        Self { url }
    }

    pub fn from_binding_path(raw_binding_icon: &str) -> Self {
        let unified_separators = raw_binding_icon.replace('\\', "/").to_ascii_lowercase();
        let trimmed_prefix = unified_separators
            .strip_prefix(REPLACEABLE_TEXTURES_PREFIX)
            .unwrap_or(&unified_separators);
        let png_path = match trimmed_prefix.strip_suffix(".blp") {
            Some(stem_without_extension) => format!("{stem_without_extension}.png"),
            None => trimmed_prefix.to_string(),
        };
        let prefix = Self::prefix();
        let url = format!("{prefix}{png_path}");
        Self { url }
    }

    pub fn from_icon_path(icon_path: &AbilityIconPath) -> Self {
        match icon_path {
            AbilityIconPath::Database(path) => Self::from_database_path(path),
            AbilityIconPath::Binding(path) => Self::from_binding_path(path),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl fmt::Display for IconUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.url)
    }
}
