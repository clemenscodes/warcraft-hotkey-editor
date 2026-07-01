use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

/// One hotkey row: the binding's display name, its section and defaults, and the
/// shared editing signal and binding map its key chip needs.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryProps {
    pub section_id: String,
    pub comment: String,
    pub default_hotkey: u32,
    pub default_modifier: SystemKeybindModifier,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
    pub binding_map: ReadSignal<SystemBindingMap>,
}
