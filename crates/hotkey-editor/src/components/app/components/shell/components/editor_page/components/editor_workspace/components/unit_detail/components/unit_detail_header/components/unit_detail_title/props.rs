use dioxus::prelude::*;

/// The title column: the name row over the unit id.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailTitleProps {
    pub unit_name: &'static str,
    pub unit_id: String,
    pub has_hero_attributes: bool,
}
