use dioxus::prelude::*;

/// The active health-regeneration leaf's input: the shaped gain text, resolved by the
/// dispatcher from the row's presentation.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainProps {
    #[props(into)]
    pub text: String,
}
