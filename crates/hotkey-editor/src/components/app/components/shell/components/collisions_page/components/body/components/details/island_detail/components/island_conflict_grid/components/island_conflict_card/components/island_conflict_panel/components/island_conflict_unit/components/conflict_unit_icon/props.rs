use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
