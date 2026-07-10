use super::view::LinkMoveNameView;
use dioxus::prelude::*;

/// The clickable ability name that deep-links into the editor; underlines on the button's hover.
#[derive(Props, Clone, PartialEq)]
pub struct LinkMoveNameProps {
    #[props(into)]
    pub text: String,
}

impl From<&LinkMoveNameView> for LinkMoveNameProps {
    fn from(view: &LinkMoveNameView) -> Self {
        let LinkMoveNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for LinkMoveNameProps {
    type View = LinkMoveNameView;
}
