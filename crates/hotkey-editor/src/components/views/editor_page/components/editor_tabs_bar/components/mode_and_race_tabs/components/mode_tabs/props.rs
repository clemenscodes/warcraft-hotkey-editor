use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// The mode column reads the current race to pick a default unit when the mode
/// changes, and writes the mode plus the reset selection.
#[derive(Props, Clone, PartialEq)]
pub struct ModeTabsProps {
    pub unit_mode: Signal<UnitMode>,
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}
