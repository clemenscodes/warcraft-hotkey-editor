use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
