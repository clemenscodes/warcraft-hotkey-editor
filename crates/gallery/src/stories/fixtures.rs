use hotkey_editor::model::icons::IconUrl;
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{CustomKeys, GridLayout};

/// A normalized empty CustomKeys (no overrides applied).
#[allow(dead_code)]
pub fn sample_keys() -> CustomKeys {
    CustomKeys::from("").normalize()
}

/// A normalized CustomKeys with the QWERTY grid applied, so every binding's
/// hotkey matches its cell's layout letter. This is the realistic state for
/// editing the command grid (after "apply grid"), in which drag-and-drop swaps
/// are non-destructive and reversible.
pub fn sample_keys_layout_applied() -> CustomKeys {
    let mut keys = sample_keys();
    keys.apply_grid_to_all_bindings(sample_grid_layout());
    keys
}

/// The standard QWERTY grid layout.
pub fn sample_grid_layout() -> GridLayout {
    GridLayout::qwerty_grid()
}

/// A real hero unit ID present in WARCRAFT_DATABASE — Human Archmage.
pub fn sample_hero_id() -> String {
    "Hamg".to_string()
}

/// A real basic unit ID present in WARCRAFT_DATABASE — Human Footman.
pub fn sample_unit_id() -> String {
    "hfoo".to_string()
}

/// A real icon for presentational previews — the Footman's icon.
pub fn sample_icon() -> Option<IconUrl> {
    let unit_object = ObjectLookup::by_id(&sample_unit_id());
    unit_object
        .and_then(|object| object.icons().first().copied())
        .map(IconUrl::from_database_path)
}

/// The same icon as a URL string.
pub fn sample_icon_url() -> String {
    sample_icon()
        .map(|icon| icon.to_string())
        .unwrap_or_default()
}
