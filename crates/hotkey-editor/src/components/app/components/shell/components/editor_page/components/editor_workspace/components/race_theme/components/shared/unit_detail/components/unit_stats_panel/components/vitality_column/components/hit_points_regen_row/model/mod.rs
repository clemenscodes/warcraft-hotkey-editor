use super::view::HitPointsRegenRowView;
use dioxus::prelude::*;
use warcraft_api::HitPointsRegen;

/// The hit-points regeneration row's input: the unit's resolved health regeneration,
/// which carries its own conditional (at night, on blight) and its rate.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRegenRowModel {
    pub value: HitPointsRegen,
}

impl From<&HitPointsRegenRowView> for HitPointsRegenRowModel {
    fn from(view: &HitPointsRegenRowView) -> Self {
        let HitPointsRegenRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for HitPointsRegenRowModel {
    type View = HitPointsRegenRowView;
}
