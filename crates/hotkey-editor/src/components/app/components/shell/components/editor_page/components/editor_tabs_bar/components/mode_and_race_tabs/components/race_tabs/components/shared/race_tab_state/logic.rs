use super::components::active_race_tab::ActiveRaceTabProps;
use super::components::inactive_race_tab::InactiveRaceTabProps;
use super::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use super::hooks::RaceTabBinding;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::RaceTabsProps;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitKindHelpers;

/// The editor selection a race click updates: the active race, plus the unit and slot
/// it resets. Grouping the navigation signals makes "apply this race" a method rather
/// than a free function.
#[derive(Clone, Copy)]
pub(super) struct RaceSelection {
    navigation: RaceTabsProps,
}

impl From<&RaceTabsProps> for RaceSelection {
    fn from(navigation: &RaceTabsProps) -> Self {
        let navigation = *navigation;
        Self { navigation }
    }
}

impl RaceSelection {
    pub(super) fn apply(self, race: Race) {
        let mut active_race = self.navigation.active_race;
        active_race.set(race);
        let unit_mode = self.navigation.unit_mode;
        let mode_value = *unit_mode.read();
        let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
        let mut selected_unit_id = self.navigation.selected_unit_id;
        selected_unit_id.set(next_id);
        let mut selected_slot = self.navigation.selected_slot;
        selected_slot.set(None);
    }
}

impl From<&RaceTabBinding> for RaceTabLabelProps {
    fn from(binding: &RaceTabBinding) -> Self {
        let label = binding.label().to_string();
        Self { label }
    }
}

impl From<&RaceTabBinding> for ActiveRaceTabProps {
    fn from(binding: &RaceTabBinding) -> Self {
        let label = RaceTabLabelProps::from(binding);
        let onclick = binding.onclick();
        let onkeydown = binding.onkeydown();
        let onmounted = binding.onmounted();
        Self {
            label,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}

impl From<&RaceTabBinding> for InactiveRaceTabProps {
    fn from(binding: &RaceTabBinding) -> Self {
        let label = RaceTabLabelProps::from(binding);
        let onclick = binding.onclick();
        let onkeydown = binding.onkeydown();
        let onmounted = binding.onmounted();
        Self {
            label,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}
