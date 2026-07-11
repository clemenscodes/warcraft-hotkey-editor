use super::view::ActiveStatGainView;
use dioxus::prelude::*;

/// The active gain leaf's input: the shaped display text, built by the dispatcher from
/// the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveStatGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveStatGainView> for ActiveStatGainModel {
    fn from(view: &ActiveStatGainView) -> Self {
        let ActiveStatGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ActiveStatGainModel {
    type View = ActiveStatGainView;
}
