use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NeutralRaceChipThemeView {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl ddd::View for NeutralRaceChipThemeView {}
