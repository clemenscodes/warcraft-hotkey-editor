use dioxus::prelude::*;

/// The unit's flavor text (its ubertip).
#[derive(Props, Clone, PartialEq)]
pub struct UnitDescriptionProps {
    #[props(into)]
    pub text: String,
}
