use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The title column: the name row over the unit id.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailTitleProps {
    pub unit_name: &'static str,
    pub unit_id: WarcraftObjectId,
    pub has_hero_attributes: bool,
}
