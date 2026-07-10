use dioxus::prelude::*;

/// The muted gain leaf's input: the shaped display text, built by the dispatcher from
/// the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct MutedStatGainProps {
    #[props(into)]
    pub text: String,
}
