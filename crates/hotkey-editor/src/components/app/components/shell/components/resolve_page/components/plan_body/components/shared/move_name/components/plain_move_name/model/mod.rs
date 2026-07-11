use super::view::PlainMoveNameView;
use dioxus::prelude::*;

/// The non-clickable ability name (no owning unit to link to).
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
