use dioxus::prelude::*;

/// The unit detail header: the portrait and the title (which, for heroes, shows the
/// level picker — the picker sources the level itself from context).
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailHeaderProps {
    pub unit_name: &'static str,
    pub unit_id: String,
    pub portrait_url: Option<String>,
    pub has_hero_attributes: bool,
}
