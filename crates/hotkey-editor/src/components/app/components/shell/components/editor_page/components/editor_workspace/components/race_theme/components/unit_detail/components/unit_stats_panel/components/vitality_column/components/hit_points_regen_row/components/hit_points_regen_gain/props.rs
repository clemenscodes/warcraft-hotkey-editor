use dioxus::prelude::*;

/// The health-regeneration gain leaf's input: the shaped gain text and whether it
/// renders muted, both resolved from the row's presentation.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenGainProps {
    #[props(into)]
    pub text: String,
    pub is_muted: bool,
}
