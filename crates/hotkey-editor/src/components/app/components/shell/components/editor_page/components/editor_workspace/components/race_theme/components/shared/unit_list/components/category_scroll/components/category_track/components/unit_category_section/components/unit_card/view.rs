use crate::components::app::components::shell::components::shared::icons::IconUrl;
use warcraft_api::{UnitKind, WarcraftObjectId};

/// The published `View` contract mirroring [`UnitCardProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardView {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
}

impl ddd::View for UnitCardView {}
