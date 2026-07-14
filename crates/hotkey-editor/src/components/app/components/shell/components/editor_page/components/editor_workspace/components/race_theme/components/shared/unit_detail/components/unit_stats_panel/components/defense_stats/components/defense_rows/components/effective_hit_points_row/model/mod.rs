use super::view::EffectiveHitPointsRowView;
use dioxus::prelude::*;
use warcraft_api::EffectiveHitPoints;

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
