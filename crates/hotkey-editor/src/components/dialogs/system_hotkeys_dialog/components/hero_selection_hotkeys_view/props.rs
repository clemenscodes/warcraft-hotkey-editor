use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the hero-selection editor needs: the loaded keys it edits and the shared
/// editing-section signal.
#[derive(Props, Clone, PartialEq)]
pub struct HeroSelectionHotkeysViewProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
}
