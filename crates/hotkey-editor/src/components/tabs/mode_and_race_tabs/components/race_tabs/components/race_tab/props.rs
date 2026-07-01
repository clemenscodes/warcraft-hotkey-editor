use super::components::race_tab_label::RaceTabLabelProps;
use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// One race tab: which race it is, whether it is active, and the shared
/// navigation signals it writes when chosen.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabProps {
    pub race: Race,
    pub is_active: bool,
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}

impl From<&RaceTabProps> for RaceTabLabelProps {
    fn from(props: &RaceTabProps) -> Self {
        let race = props.race;
        let label = RaceLabels::display_name(race).to_string();
        Self { race, label }
    }
}
