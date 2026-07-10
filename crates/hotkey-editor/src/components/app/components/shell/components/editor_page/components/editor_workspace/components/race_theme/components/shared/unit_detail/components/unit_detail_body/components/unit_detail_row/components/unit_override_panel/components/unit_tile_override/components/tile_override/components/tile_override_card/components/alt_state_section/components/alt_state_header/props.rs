use dioxus::prelude::*;

/// The off-state block's top row: the label column beside its editable controls.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderProps {
    pub alt_name_text: Option<String>,
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}
