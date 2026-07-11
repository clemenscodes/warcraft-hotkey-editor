use super::view::ActiveStatValueView;
use dioxus::prelude::*;

/// The active value leaf's input: the shaped display text, built by the dispatcher
/// from the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveStatValueModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveStatValueView> for ActiveStatValueModel {
    fn from(view: &ActiveStatValueView) -> Self {
        let ActiveStatValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ActiveStatValueModel {
    type View = ActiveStatValueView;
}
