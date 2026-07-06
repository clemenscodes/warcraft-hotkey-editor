use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap, WarcraftObjectId};

/// A key chip for a list-view hotkey row: the section it binds, the loaded keys it
/// edits, the shared editing signal, and the resolved binding map.
#[derive(Props, Clone, PartialEq)]
pub struct KeyCaptureCellProps {
    pub section_id: WarcraftObjectId,
    pub default_hotkey: u32,
    pub default_modifier: SystemKeybindModifier,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub binding_map: ReadSignal<SystemBindingMap>,
}
