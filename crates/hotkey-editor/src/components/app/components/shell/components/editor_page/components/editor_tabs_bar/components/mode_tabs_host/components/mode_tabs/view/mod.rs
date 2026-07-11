use dioxus::prelude::*;
use warcraft_api::UnitMode;

/// The published `View` contract mirroring [`ModeTabsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ModeTabsView {
    pub unit_mode: Signal<UnitMode>,
    pub on_select: EventHandler<UnitMode>,
}

impl ddd::View for ModeTabsView {}
