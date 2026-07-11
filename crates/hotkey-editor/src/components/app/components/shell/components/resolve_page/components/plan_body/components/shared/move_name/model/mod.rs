use super::view::MoveNameView;
use dioxus::prelude::*;
/// A moved/rival ability's name; `is_link` underlines it on the button's hover.
#[derive(Props, Clone, PartialEq)]
pub struct MoveNameModel {
    #[props(into)]
    pub text: String,
    pub is_link: bool,
}

impl From<&MoveNameView> for MoveNameModel {
    fn from(view: &MoveNameView) -> Self {
        let MoveNameView { text, is_link } = view.clone();
        Self { text, is_link }
    }
}

impl ddd::Model for MoveNameModel {
    type View = MoveNameView;
}
