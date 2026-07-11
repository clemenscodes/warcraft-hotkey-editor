use super::view::AltStateHeaderView;
use dioxus::prelude::*;

/// The off-state block's top row: the label column beside its editable controls.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderModel {
    pub alt_name_text: Option<String>,
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&AltStateHeaderView> for AltStateHeaderModel {
    fn from(view: &AltStateHeaderView) -> Self {
        let AltStateHeaderView {
            alt_name_text,
            show,
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        } = view.clone();
        Self {
            alt_name_text,
            show,
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        }
    }
}

impl ddd::Model for AltStateHeaderModel {
    type View = AltStateHeaderView;
}
