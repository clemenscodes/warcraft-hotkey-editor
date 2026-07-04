use dioxus::prelude::*;

/// One description line of the alt-state block.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateLineProps {
    #[props(into)]
    pub text: String,
}
