use super::view::HitPointsRowView;
use dioxus::prelude::*;
use warcraft_api::HitPoints;

#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRowModel {
    pub value: HitPoints,
}

impl From<&HitPointsRowView> for HitPointsRowModel {
    fn from(view: &HitPointsRowView) -> Self {
        let HitPointsRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for HitPointsRowModel {
    type View = HitPointsRowView;
}
