use dioxus::prelude::*;

/// The muted value leaf's input: the shaped display text, built by the dispatcher
/// from the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct MutedStatValueProps {
    #[props(into)]
    pub text: String,
}
