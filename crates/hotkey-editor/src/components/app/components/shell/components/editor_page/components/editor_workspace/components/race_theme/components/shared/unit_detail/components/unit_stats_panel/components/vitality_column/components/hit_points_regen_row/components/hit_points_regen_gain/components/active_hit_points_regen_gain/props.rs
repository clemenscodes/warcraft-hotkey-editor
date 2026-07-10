use super::view::ActiveHitPointsRegenGainView;
use dioxus::prelude::*;

/// The active health-regeneration leaf's input: the shaped gain text, resolved by the
/// dispatcher from the row's presentation.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainProps {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveHitPointsRegenGainView> for ActiveHitPointsRegenGainProps {
    fn from(view: &ActiveHitPointsRegenGainView) -> Self {
        let ActiveHitPointsRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ActiveHitPointsRegenGainProps {
    type View = ActiveHitPointsRegenGainView;
}
