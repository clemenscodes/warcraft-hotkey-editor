use dioxus::prelude::*;

/// The name row: the unit name, and — for heroes — the level picker beside it.
#[derive(Props, Clone, PartialEq)]
pub struct UnitNameRowProps {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}
