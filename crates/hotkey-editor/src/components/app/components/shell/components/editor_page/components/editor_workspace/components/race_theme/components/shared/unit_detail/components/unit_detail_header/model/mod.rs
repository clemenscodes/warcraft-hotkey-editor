use super::view::UnitDetailHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailHeaderModel {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub portrait_url: Option<String>,
    pub has_hero_attributes: bool,
}

impl From<&UnitDetailHeaderView> for UnitDetailHeaderModel {
    fn from(view: &UnitDetailHeaderView) -> Self {
        let UnitDetailHeaderView {
            unit_name,
            unit_id,
            portrait_url,
            has_hero_attributes,
        } = view.clone();
        Self {
            unit_name,
            unit_id,
            portrait_url,
            has_hero_attributes,
        }
    }
}

impl ddd::Model for UnitDetailHeaderModel {
    type View = UnitDetailHeaderView;
}
