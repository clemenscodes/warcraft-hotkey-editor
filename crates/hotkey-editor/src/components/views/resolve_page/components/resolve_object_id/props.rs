use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ResolveObjectIdProps {
    #[props(into)]
    pub text: String,
}
