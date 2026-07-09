use dioxus::prelude::*;

/// The clickable ability name that deep-links into the editor; underlines on the button's hover.
#[derive(Props, Clone, PartialEq)]
pub struct LinkMoveNameProps {
    #[props(into)]
    pub text: String,
}
