use dioxus::prelude::*;
/// A moved/rival ability's name; `is_link` underlines it on the button's hover.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveMoveNameProps {
    #[props(into)]
    pub text: String,
    pub is_link: bool,
}
