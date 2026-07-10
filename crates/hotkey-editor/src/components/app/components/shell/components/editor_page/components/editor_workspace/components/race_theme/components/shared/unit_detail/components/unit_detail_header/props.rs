use super::view::UnitDetailHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit detail header: the portrait and the title (which, for heroes, shows the
/// level picker — the picker sources the level itself from context).
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailHeaderProps {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub portrait_url: Option<String>,
    pub has_hero_attributes: bool,
}

impl From<&UnitDetailHeaderView> for UnitDetailHeaderProps {
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

impl ddd::Props for UnitDetailHeaderProps {
    type View = UnitDetailHeaderView;
}
