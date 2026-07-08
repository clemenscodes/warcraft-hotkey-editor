use dioxus::prelude::*;

/// The unit id text and the `data-race` attribute value the code element carries.
#[derive(Props, Clone, PartialEq)]
pub struct NormalUnitCardIdProps {
    pub race_attribute: &'static str,
    #[props(into)]
    pub text: String,
}
