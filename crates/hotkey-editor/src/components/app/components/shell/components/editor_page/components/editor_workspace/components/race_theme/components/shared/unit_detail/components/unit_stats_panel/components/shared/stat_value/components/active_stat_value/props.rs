use dioxus::prelude::*;

/// The active value leaf's input: the shaped display text, built by the dispatcher
/// from the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveStatValueProps {
    #[props(into)]
    pub text: String,
}
