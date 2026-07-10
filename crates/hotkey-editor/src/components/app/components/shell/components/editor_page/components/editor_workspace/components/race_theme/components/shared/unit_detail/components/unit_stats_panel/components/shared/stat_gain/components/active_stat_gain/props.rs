use dioxus::prelude::*;

/// The active gain leaf's input: the shaped display text, built by the dispatcher from
/// the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveStatGainProps {
    #[props(into)]
    pub text: String,
}
