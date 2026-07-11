use dioxus::prelude::*;
use warcraft_api::Race;

/// The published `View` contract mirroring [`RaceTabsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RaceTabsView {
    pub active_race: Signal<Race>,
    pub on_select: EventHandler<Race>,
}

impl ddd::View for RaceTabsView {}
