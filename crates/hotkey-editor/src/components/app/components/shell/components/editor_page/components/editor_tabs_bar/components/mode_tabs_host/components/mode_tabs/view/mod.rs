use dioxus::prelude::*;
use warcraft_api::UnitMode;

#[derive(Clone, PartialEq)]
pub struct ModeTabsView {
    pub unit_mode: Signal<UnitMode>,
    pub on_select: EventHandler<UnitMode>,
}

impl ddd::View for ModeTabsView {}
