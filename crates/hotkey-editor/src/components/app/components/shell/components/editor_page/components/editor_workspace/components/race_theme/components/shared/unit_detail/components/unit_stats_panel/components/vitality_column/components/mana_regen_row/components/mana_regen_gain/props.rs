use super::view::ManaRegenGainView;
use dioxus::prelude::*;
use warcraft_keybinds::ManaRegen;

/// The mana-regeneration gain leaf's input: the unit's resolved mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenGainProps {
    pub value: ManaRegen,
}

impl From<&ManaRegenGainView> for ManaRegenGainProps {
    fn from(view: &ManaRegenGainView) -> Self {
        let ManaRegenGainView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for ManaRegenGainProps {
    type View = ManaRegenGainView;
}
