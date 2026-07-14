use super::view::LinkMoveNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LinkMoveNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&LinkMoveNameView> for LinkMoveNameModel {
    fn from(view: &LinkMoveNameView) -> Self {
        let LinkMoveNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for LinkMoveNameModel {
    type View = LinkMoveNameView;
}
