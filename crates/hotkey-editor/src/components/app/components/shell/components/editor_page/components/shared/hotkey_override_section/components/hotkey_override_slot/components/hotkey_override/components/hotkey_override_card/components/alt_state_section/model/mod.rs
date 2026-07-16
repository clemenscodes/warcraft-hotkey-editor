use super::view::AltStateSectionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStateSectionModel {
    pub alt_name_text: Option<String>,
    pub alt_description_lines: Vec<String>,
    pub show_alt_controls: bool,
    pub alt_hotkey_label: String,
    pub alt_hotkey_is_editing: bool,
    pub alt_hotkey_is_special_token: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&AltStateSectionView> for AltStateSectionModel {
    fn from(view: &AltStateSectionView) -> Self {
        let AltStateSectionView {
            alt_name_text,
            alt_description_lines,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_position_click,
            on_hotkey_activate,
        } = view.clone();
        Self {
            alt_name_text,
            alt_description_lines,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_position_click,
            on_hotkey_activate,
        }
    }
}

impl ddd::Model for AltStateSectionModel {
    type View = AltStateSectionView;
}
