use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyDetailUnitIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
