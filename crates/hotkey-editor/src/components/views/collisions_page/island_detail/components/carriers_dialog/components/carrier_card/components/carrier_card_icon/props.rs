use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardIconProps {
    #[props(into)]
    pub src: String,
    #[props(into)]
    pub alt: String,
}
