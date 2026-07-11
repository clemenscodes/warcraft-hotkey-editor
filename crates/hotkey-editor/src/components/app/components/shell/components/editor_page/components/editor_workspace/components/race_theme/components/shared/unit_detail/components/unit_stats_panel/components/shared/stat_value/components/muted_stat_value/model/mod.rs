use super::view::MutedStatValueView;
use dioxus::prelude::*;

/// The muted value leaf's input: the shaped display text, built by the dispatcher
/// from the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct MutedStatValueModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedStatValueView> for MutedStatValueModel {
    fn from(view: &MutedStatValueView) -> Self {
        let MutedStatValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedStatValueModel {
    type View = MutedStatValueView;
}
