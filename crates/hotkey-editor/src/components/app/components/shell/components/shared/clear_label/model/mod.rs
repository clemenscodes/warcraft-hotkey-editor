use super::view::ClearLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClearLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&ClearLabelView> for ClearLabelModel {
    fn from(view: &ClearLabelView) -> Self {
        let ClearLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ClearLabelModel {
    type View = ClearLabelView;
}
