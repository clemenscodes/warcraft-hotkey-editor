use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ResolveEmptyMessageProps {
    #[props(into)]
    pub text: String,
}
