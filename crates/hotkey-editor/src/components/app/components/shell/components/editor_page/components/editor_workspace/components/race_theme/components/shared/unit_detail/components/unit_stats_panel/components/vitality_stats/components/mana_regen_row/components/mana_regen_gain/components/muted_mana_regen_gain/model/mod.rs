use super::view::MutedManaRegenGainView;
use dioxus::prelude::*;

/// The muted mana-regeneration leaf's input: the shaped display text, built by the
/// dispatcher from the unit's mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct MutedManaRegenGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedManaRegenGainView> for MutedManaRegenGainModel {
    fn from(view: &MutedManaRegenGainView) -> Self {
        let MutedManaRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedManaRegenGainModel {
    type View = MutedManaRegenGainView;
}
