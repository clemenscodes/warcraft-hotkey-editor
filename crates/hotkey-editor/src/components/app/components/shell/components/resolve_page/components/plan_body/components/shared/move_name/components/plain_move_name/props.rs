use dioxus::prelude::*;

/// The non-clickable ability name (no owning unit to link to).
#[derive(Props, Clone, PartialEq)]
pub struct PlainMoveNameProps {
    #[props(into)]
    pub text: String,
}
