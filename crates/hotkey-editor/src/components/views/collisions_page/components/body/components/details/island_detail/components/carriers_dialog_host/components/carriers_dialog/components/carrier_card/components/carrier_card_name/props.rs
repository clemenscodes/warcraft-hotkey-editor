use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardNameProps {
    #[props(into)]
    pub text: String,
}
