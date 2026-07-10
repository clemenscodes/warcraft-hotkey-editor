use dioxus::prelude::*;

/// The right side of the override header: exactly one of the hotkey cell, the
/// research hotkey cell, or the passive note (or nothing), chosen by the flags.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHotkeySlotProps {
    pub show_hotkey_field: bool,
    pub hotkey_label: String,
    pub hotkey_is_editing: bool,
    pub hotkey_is_special: bool,
    pub on_hotkey_activate: EventHandler<()>,
    pub show_research_field: bool,
    pub research_label: String,
    pub research_is_editing: bool,
    pub research_is_special: bool,
    pub on_research_activate: EventHandler<()>,
    pub is_info_only: bool,
}
