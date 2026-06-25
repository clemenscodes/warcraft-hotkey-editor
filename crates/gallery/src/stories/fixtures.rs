use warcraft_keybinds::{CustomKeys, GridLayout};

/// A normalized empty CustomKeys (no overrides applied).
#[allow(dead_code)]
pub fn sample_keys() -> CustomKeys {
    CustomKeys::from("").normalize()
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
