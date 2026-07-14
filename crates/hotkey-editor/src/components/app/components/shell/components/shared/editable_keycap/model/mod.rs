use super::state::EditableKeycapState;
use super::view::EditableKeycapView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditableKeycapModel {
    #[props(into)]
    pub label: String,
    #[props(default)]
    pub state: EditableKeycapState,
}

impl From<&EditableKeycapView> for EditableKeycapModel {
    fn from(view: &EditableKeycapView) -> Self {
        let EditableKeycapView { label, state } = view.clone();
        Self { label, state }
    }
}

impl ddd::Model for EditableKeycapModel {
    type View = EditableKeycapView;
}
