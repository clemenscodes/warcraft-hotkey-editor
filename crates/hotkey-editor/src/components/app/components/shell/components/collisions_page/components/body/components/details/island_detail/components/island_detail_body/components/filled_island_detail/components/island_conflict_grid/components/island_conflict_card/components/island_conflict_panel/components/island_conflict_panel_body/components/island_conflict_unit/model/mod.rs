use super::view::IslandConflictUnitView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitModel {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}

impl From<&IslandConflictUnitView> for IslandConflictUnitModel {
    fn from(view: &IslandConflictUnitView) -> Self {
        let IslandConflictUnitView {
            unit_id,
            icon_url,
            name,
        } = view.clone();
        Self {
            unit_id,
            icon_url,
            name,
        }
    }
}

impl ddd::Model for IslandConflictUnitModel {
    type View = IslandConflictUnitView;
}
