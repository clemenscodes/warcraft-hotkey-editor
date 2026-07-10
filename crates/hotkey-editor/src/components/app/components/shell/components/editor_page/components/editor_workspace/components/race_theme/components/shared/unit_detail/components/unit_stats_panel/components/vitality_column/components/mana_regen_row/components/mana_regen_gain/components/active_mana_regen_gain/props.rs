use dioxus::prelude::*;

/// The active mana-regeneration leaf's input: the shaped display text, built by the
/// dispatcher from the unit's mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveManaRegenGainProps {
    #[props(into)]
    pub text: String,
}
