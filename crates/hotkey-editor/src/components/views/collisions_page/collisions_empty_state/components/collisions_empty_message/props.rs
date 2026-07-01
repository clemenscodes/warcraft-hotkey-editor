use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsEmptyMessageProps {
    #[props(into)]
    pub text: String,
}
