use super::view::MutedStatGainView;
use dioxus::prelude::*;

/// The muted gain leaf's input: the shaped display text, built by the dispatcher from
/// the domain figure.
#[derive(Props, Clone, PartialEq)]
pub struct MutedStatGainProps {
    #[props(into)]
    pub text: String,
}

impl From<&MutedStatGainView> for MutedStatGainProps {
    fn from(view: &MutedStatGainView) -> Self {
        let MutedStatGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for MutedStatGainProps {
    type View = MutedStatGainView;
}
