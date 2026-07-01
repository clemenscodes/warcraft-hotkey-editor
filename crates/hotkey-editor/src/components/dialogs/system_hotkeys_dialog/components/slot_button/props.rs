use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

/// A big WC3 slot used in the hero-selection and control-group layouts (edit on
/// click, no drag). `compact` marks the tighter control-group cell.
#[derive(Props, Clone, PartialEq)]
pub struct SlotButtonProps {
    pub slot_label: String,
    pub section_id: String,
    pub default_hotkey: u32,
    pub default_modifier: SystemKeybindModifier,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
    pub binding_map: ReadSignal<SystemBindingMap>,
    #[props(default = false)]
    pub compact: bool,
}
