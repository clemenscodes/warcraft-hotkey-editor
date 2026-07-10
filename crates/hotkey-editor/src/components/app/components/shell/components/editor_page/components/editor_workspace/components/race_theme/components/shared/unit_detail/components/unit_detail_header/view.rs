use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitDetailHeaderProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDetailHeaderView {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub portrait_url: Option<String>,
    pub has_hero_attributes: bool,
}

impl ddd::View for UnitDetailHeaderView {}
