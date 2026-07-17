use dioxus::prelude::*;
use warcraft_api::UnitMode;
use warcraft_api::UnitModeSelection;

#[derive(Clone, PartialEq)]
pub struct ModeTabsView {
    pub unit_modes: Signal<UnitModeSelection>,
    pub on_select: EventHandler<UnitMode>,
}

impl ddd::View for ModeTabsView {}
