use super::view::ManaRegenGainView;
use dioxus::prelude::*;
use warcraft_api::ManaRegen;

/// The mana-regeneration gain leaf's input: the unit's resolved mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenGainModel {
    pub value: ManaRegen,
}

impl From<&ManaRegenGainView> for ManaRegenGainModel {
    fn from(view: &ManaRegenGainView) -> Self {
        let ManaRegenGainView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for ManaRegenGainModel {
    type View = ManaRegenGainView;
}
