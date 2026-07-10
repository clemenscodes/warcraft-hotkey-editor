use super::view::ActiveManaRegenGainView;
use dioxus::prelude::*;

/// The active mana-regeneration leaf's input: the shaped display text, built by the
/// dispatcher from the unit's mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveManaRegenGainProps {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveManaRegenGainView> for ActiveManaRegenGainProps {
    fn from(view: &ActiveManaRegenGainView) -> Self {
        let ActiveManaRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ActiveManaRegenGainProps {
    type View = ActiveManaRegenGainView;
}
