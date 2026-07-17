use crate::components::app::components::shell::components::shared::icons::IconUrl;
use warcraft_api::{UnitKind, WarcraftObjectId};

#[derive(Clone, PartialEq)]
pub struct UnitCardView {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
}

impl ddd::View for UnitCardView {}
