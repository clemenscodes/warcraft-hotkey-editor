use dioxus::prelude::*;

/// The unit's database id, shown as a monospace caption.
#[derive(Props, Clone, PartialEq)]
pub struct UnitIdProps {
    #[props(into)]
    pub text: String,
}
