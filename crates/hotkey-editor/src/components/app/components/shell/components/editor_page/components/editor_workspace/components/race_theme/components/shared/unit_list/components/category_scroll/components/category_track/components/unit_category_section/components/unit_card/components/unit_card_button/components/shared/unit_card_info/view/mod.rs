use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitCardInfoModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardInfoView {
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl ddd::View for UnitCardInfoView {}
