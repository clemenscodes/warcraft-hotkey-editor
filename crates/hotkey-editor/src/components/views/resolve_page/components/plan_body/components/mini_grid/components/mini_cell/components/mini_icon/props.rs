use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct MiniIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
