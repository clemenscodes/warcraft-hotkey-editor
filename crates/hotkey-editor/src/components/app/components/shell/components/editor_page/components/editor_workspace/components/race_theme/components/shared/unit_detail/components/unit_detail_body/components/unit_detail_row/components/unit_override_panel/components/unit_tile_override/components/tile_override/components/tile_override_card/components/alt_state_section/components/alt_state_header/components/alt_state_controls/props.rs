use dioxus::prelude::*;

/// The editable controls of the off-state block: whether they show at all, plus the
/// off-state hotkey data and the two handlers.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateControlsProps {
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}
