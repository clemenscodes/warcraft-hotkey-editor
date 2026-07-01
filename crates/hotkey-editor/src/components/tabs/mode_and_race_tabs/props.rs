use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// The shared unit-navigation state: the current mode and race, and the selection
/// the mode and race tabs reset when the player switches context.
#[derive(Props, Clone, PartialEq)]
pub struct ModeAndRaceTabsProps {
    pub unit_mode: Signal<UnitMode>,
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}
