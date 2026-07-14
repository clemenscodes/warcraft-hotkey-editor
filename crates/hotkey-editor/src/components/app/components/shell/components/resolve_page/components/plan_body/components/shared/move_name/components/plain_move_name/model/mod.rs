use super::view::PlainMoveNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PlainMoveNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&PlainMoveNameView> for PlainMoveNameModel {
    fn from(view: &PlainMoveNameView) -> Self {
        let PlainMoveNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for PlainMoveNameModel {
    type View = PlainMoveNameView;
}
