use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Clone, PartialEq)]
pub struct RaceTabsView {
    pub active_race: Race,
    pub on_select: EventHandler<Race>,
}

impl ddd::Layered for RaceTabsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceTabsView {}
