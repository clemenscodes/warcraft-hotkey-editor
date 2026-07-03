use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// The race tabs read the active race to mark the current tab and reset the
/// selection when the player switches race.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabsProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}
