use super::view::OverrideKeyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OverrideKeyModel {
    #[props(into)]
    pub label: String,
    pub is_editing: bool,
    pub is_special: bool,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl From<&OverrideKeyView> for OverrideKeyModel {
    fn from(view: &OverrideKeyView) -> Self {
        let OverrideKeyView {
            label,
            is_editing,
            is_special,
            title,
            on_activate,
        } = view.clone();
        Self {
            label,
            is_editing,
            is_special,
            title,
            on_activate,
        }
    }
}

impl ddd::Model for OverrideKeyModel {
    type View = OverrideKeyView;
}
