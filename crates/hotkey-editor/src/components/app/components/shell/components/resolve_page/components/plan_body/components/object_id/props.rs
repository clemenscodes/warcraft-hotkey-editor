use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ObjectIdProps {
    #[props(into)]
    pub text: String,
}
