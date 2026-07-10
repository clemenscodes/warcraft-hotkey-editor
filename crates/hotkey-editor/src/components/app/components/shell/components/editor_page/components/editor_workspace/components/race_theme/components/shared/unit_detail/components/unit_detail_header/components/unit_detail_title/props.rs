use super::view::UnitDetailTitleView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The title column: the name row over the unit id.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailTitleProps {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub has_hero_attributes: bool,
}

impl From<&UnitDetailTitleView> for UnitDetailTitleProps {
    fn from(view: &UnitDetailTitleView) -> Self {
        let UnitDetailTitleView {
            unit_name,
            unit_id,
            has_hero_attributes,
        } = view.clone();
        Self {
            unit_name,
            unit_id,
            has_hero_attributes,
        }
    }
}

impl ddd::Props for UnitDetailTitleProps {
    type View = UnitDetailTitleView;
}
