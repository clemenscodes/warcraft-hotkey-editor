use super::view::EffectiveHitPointsRowView;
use dioxus::prelude::*;
use warcraft_keybinds::EffectiveHitPoints;

/// The effective hit points row's input: raw health scaled by armor.
#[derive(Props, Clone, PartialEq)]
pub struct EffectiveHitPointsRowModel {
    pub value: EffectiveHitPoints,
}

impl From<&EffectiveHitPointsRowView> for EffectiveHitPointsRowModel {
    fn from(view: &EffectiveHitPointsRowView) -> Self {
        let EffectiveHitPointsRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for EffectiveHitPointsRowModel {
    type View = EffectiveHitPointsRowView;
}
