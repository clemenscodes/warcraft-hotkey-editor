use super::view::SpecialOverrideKeyView;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SpecialOverrideKeyModel {
    #[props(into)]
    pub label: String,
    pub state: EditableKeycapState,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl From<&SpecialOverrideKeyView> for SpecialOverrideKeyModel {
    fn from(view: &SpecialOverrideKeyView) -> Self {
        let SpecialOverrideKeyView {
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

impl ddd::Model for SpecialOverrideKeyModel {
    type View = SpecialOverrideKeyView;
}
