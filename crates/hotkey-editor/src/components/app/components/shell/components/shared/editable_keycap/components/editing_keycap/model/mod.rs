use super::view::EditingKeycapView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditingKeycapModel {
    #[props(into)]
    pub label: String,
}

impl From<&EditingKeycapView> for EditingKeycapModel {
    fn from(view: &EditingKeycapView) -> Self {
        let EditingKeycapView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for EditingKeycapModel {
    type View = EditingKeycapView;
}
