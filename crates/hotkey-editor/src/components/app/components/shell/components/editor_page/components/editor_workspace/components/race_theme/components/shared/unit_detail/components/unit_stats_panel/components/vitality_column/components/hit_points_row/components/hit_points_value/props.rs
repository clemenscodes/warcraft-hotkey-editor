use super::view::HitPointsValueView;
use dioxus::prelude::*;
use warcraft_keybinds::HitPoints;

/// The hit-points value leaf's input: the unit's resolved hit points.
#[derive(Props, Clone, PartialEq)]
pub struct HitPointsValueProps {
    pub value: HitPoints,
}

impl From<&HitPointsValueView> for HitPointsValueProps {
    fn from(view: &HitPointsValueView) -> Self {
        let HitPointsValueView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for HitPointsValueProps {
    type View = HitPointsValueView;
}
