use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitDetailTitleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDetailTitleView {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub has_hero_attributes: bool,
}

impl ddd::View for UnitDetailTitleView {}
