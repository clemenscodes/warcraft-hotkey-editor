use super::view::AltStateControlsView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStateControlsModel {
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&AltStateControlsView> for AltStateControlsModel {
    fn from(view: &AltStateControlsView) -> Self {
        let AltStateControlsView {
            show,
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        } = view.clone();
        Self {
            show,
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        }
    }
}

impl ddd::Model for AltStateControlsModel {
    type View = AltStateControlsView;
}
