use super::components::race_tab::RaceTabProps;
use super::props::RaceTabsProps;
use dioxus::prelude::*;
use warcraft_api::SUPPORTED_RACES;

/// The finished race tabs in supported-race order, each carrying its active flag
/// and the shared navigation signals.
pub(super) struct RaceTabList {
    pub(super) tabs: Vec<RaceTabProps>,
}

impl From<&RaceTabsProps> for RaceTabList {
    fn from(props: &RaceTabsProps) -> Self {
        let current_race = *props.active_race.read();
        let tabs = SUPPORTED_RACES
            .iter()
            .copied()
            .map(|race| {
                let is_active = current_race == race;
                let active_race = props.active_race;
                let unit_mode = props.unit_mode;
                let selected_unit_id = props.selected_unit_id;
                let selected_slot = props.selected_slot;
                RaceTabProps {
                    race,
                    is_active,
                    active_race,
                    unit_mode,
                    selected_unit_id,
                    selected_slot,
                }
            })
            .collect();
        Self { tabs }
    }
}
