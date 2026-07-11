use super::view::ConflictDetailUnitView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitModel {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}

impl From<&ConflictDetailUnitView> for ConflictDetailUnitModel {
    fn from(view: &ConflictDetailUnitView) -> Self {
        let ConflictDetailUnitView {
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

impl ddd::Model for ConflictDetailUnitModel {
    type View = ConflictDetailUnitView;
}
