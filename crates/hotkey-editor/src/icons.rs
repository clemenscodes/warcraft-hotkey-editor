use std::fmt;

use warcraft_keybinds::AbilityIconPath;

// Must match the `base_path` value in `Dioxus.toml`. The Dioxus
// `asset!()` macro injects this prefix automatically, but URLs we
// hand-build for `public/`-served files (BTN command-button icons,
// unit portraits) bypass that machinery and need it spelled out.
//
// Keep the leading slash so the URL is anchored to the document root —
// without it, a deep route like `/warcraft-hotkey-editor/foo/bar`
// would resolve `icons/...` against `/warcraft-hotkey-editor/foo/`
// and 404.
const ICON_URL_PREFIX: &str = "/warcraft-hotkey-editor/icons/";
const REPLACEABLE_TEXTURES_PREFIX: &str = "replaceabletextures/";

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct IconUrl {
    url: String,
}

impl IconUrl {
    pub(crate) fn from_database_path(database_icon_path: &str) -> Self {
        let lowered_path = database_icon_path.to_ascii_lowercase();
        let png_path = match lowered_path.strip_suffix(".blp") {
            Some(stem_without_extension) => format!("{stem_without_extension}.png"),
            None => lowered_path,
        };
        let url = format!("{ICON_URL_PREFIX}{png_path}");
        Self { url }
    }

    pub(crate) fn from_binding_path(raw_binding_icon: &str) -> Self {
        let unified_separators = raw_binding_icon.replace('\\', "/").to_ascii_lowercase();
        let trimmed_prefix = unified_separators
            .strip_prefix(REPLACEABLE_TEXTURES_PREFIX)
            .unwrap_or(&unified_separators);
        let png_path = match trimmed_prefix.strip_suffix(".blp") {
            Some(stem_without_extension) => format!("{stem_without_extension}.png"),
            None => trimmed_prefix.to_string(),
        };
        let url = format!("{ICON_URL_PREFIX}{png_path}");
        Self { url }
    }

    pub(crate) fn from_icon_path(icon_path: &AbilityIconPath) -> Self {
        match icon_path {
            AbilityIconPath::Database(path) => Self::from_database_path(path),
            AbilityIconPath::Binding(path) => Self::from_binding_path(path),
        }
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }
}

impl fmt::Display for IconUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.url)
    }
}
