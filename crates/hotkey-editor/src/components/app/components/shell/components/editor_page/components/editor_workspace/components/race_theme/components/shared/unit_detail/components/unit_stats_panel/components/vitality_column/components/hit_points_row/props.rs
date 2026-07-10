use super::view::HitPointsRowView;
use dioxus::prelude::*;
use warcraft_keybinds::HitPoints;

/// The hit points row's input: the unit's resolved hit points at the selected level.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRowProps {
    pub value: HitPoints,
}

impl From<&HitPointsRowView> for HitPointsRowProps {
    fn from(view: &HitPointsRowView) -> Self {
        let HitPointsRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for HitPointsRowProps {
    type View = HitPointsRowView;
}
