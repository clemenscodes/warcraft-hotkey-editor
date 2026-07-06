use dioxus::prelude::*;
use warcraft_api::Race;

/// The unit detail panel's inputs: the active race and the selected unit. The editor
/// signals its grids and override card drive are sourced from context by those
/// children's own hooks, so the panel is fed no god-bag of signals as props.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailPanelProps {
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
}
