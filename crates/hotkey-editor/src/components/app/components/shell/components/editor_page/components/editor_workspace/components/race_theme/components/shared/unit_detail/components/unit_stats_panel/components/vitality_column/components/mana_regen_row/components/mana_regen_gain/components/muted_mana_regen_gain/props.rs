use super::view::MutedManaRegenGainView;
use dioxus::prelude::*;

/// The muted mana-regeneration leaf's input: the shaped display text, built by the
/// dispatcher from the unit's mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct MutedManaRegenGainProps {
    #[props(into)]
    pub text: String,
}

impl From<&MutedManaRegenGainView> for MutedManaRegenGainProps {
    fn from(view: &MutedManaRegenGainView) -> Self {
        let MutedManaRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for MutedManaRegenGainProps {
    type View = MutedManaRegenGainView;
}
