use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OrcRaceChipThemeView {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl ddd::View for OrcRaceChipThemeView {}
