use super::view::MutedStatValueView;
use dioxus::prelude::*;

/// The muted value leaf's input: the shaped display text, built by the dispatcher
/// from the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct MutedStatValueProps {
    #[props(into)]
    pub text: String,
}

impl From<&MutedStatValueView> for MutedStatValueProps {
    fn from(view: &MutedStatValueView) -> Self {
        let MutedStatValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for MutedStatValueProps {
    type View = MutedStatValueView;
}
