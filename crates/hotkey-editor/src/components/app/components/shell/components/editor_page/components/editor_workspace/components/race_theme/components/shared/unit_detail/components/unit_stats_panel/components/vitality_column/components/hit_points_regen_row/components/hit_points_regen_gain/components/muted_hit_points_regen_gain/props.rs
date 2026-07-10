use super::view::MutedHitPointsRegenGainView;
use dioxus::prelude::*;

/// The muted health-regeneration leaf's input: the shaped gain text, resolved by the
/// dispatcher from the row's presentation.
#[derive(Props, Clone, PartialEq)]
pub struct MutedHitPointsRegenGainProps {
    #[props(into)]
    pub text: String,
}

impl From<&MutedHitPointsRegenGainView> for MutedHitPointsRegenGainProps {
    fn from(view: &MutedHitPointsRegenGainView) -> Self {
        let MutedHitPointsRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for MutedHitPointsRegenGainProps {
    type View = MutedHitPointsRegenGainView;
}
