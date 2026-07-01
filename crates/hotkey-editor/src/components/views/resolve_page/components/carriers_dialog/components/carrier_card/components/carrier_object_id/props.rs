use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierObjectIdProps {
    #[props(into)]
    pub text: String,
}
