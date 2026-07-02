use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictCardCaptionProps {
    #[props(into)]
    pub text: String,
}
