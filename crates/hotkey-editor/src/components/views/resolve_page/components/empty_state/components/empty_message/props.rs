use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct EmptyMessageProps {
    #[props(into)]
    pub text: String,
}
