use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Clone, PartialEq)]
pub struct RaceChipThemeView {
    pub race: Race,
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl ddd::View for RaceChipThemeView {}
