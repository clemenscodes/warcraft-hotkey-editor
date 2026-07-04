use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
