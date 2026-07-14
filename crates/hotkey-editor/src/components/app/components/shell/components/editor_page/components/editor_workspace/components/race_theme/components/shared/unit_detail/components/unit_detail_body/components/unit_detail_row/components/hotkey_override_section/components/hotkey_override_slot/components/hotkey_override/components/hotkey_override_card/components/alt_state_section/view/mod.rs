use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AltStateSectionView {
    pub alt_name_text: Option<String>,
    pub alt_description_lines: Vec<String>,
    pub show_alt_controls: bool,
    pub alt_hotkey_label: String,
    pub alt_hotkey_is_editing: bool,
    pub alt_hotkey_is_special_token: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for AltStateSectionView {}
