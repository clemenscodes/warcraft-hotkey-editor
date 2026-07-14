use super::view::NormalOverrideKeyView;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NormalOverrideKeyModel {
    #[props(into)]
    pub label: String,
    pub state: EditableKeycapState,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl From<&NormalOverrideKeyView> for NormalOverrideKeyModel {
    fn from(view: &NormalOverrideKeyView) -> Self {
        let NormalOverrideKeyView {
            label,
            state,
            title,
            on_activate,
        } = view.clone();
        Self {
            label,
            state,
            title,
            on_activate,
        }
    }
}

impl ddd::Model for NormalOverrideKeyModel {
    type View = NormalOverrideKeyView;
}
