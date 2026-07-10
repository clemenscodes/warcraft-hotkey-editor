use dioxus::prelude::*;

/// The upgraded-form block: the position button and hotkey cell for the unit that
/// shares this button position after an upgrade (e.g. post-Barrage Siege Engine).
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionProps {
    /// Whether this ability has an upgraded form to show controls for.
    pub show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}
