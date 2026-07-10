use super::view::ConflictDetailUnitView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitProps {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}

impl From<&ConflictDetailUnitView> for ConflictDetailUnitProps {
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

impl ddd::Props for ConflictDetailUnitProps {
    type View = ConflictDetailUnitView;
}
