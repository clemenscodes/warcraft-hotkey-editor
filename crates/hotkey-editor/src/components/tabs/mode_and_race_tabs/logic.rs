use super::components::mode_tabs::ModeTabsProps;
use super::components::race_tabs::RaceTabsProps;
use super::props::ModeAndRaceTabsProps;

impl From<&ModeAndRaceTabsProps> for ModeTabsProps {
    fn from(props: &ModeAndRaceTabsProps) -> Self {
        let unit_mode = props.unit_mode;
        let active_race = props.active_race;
        let selected_unit_id = props.selected_unit_id;
        let selected_slot = props.selected_slot;
        Self {
            unit_mode,
            active_race,
            selected_unit_id,
            selected_slot,
        }
    }
}

impl From<&ModeAndRaceTabsProps> for RaceTabsProps {
    fn from(props: &ModeAndRaceTabsProps) -> Self {
        let active_race = props.active_race;
        let unit_mode = props.unit_mode;
        let selected_unit_id = props.selected_unit_id;
        let selected_slot = props.selected_slot;
        Self {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
        }
    }
}
