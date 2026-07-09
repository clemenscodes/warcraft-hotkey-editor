use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// The unit-navigation signals the race tabs share: the active race each tab compares
/// against to know whether it is the current tab, plus the mode and selection a race
/// click resets. This one bundle is threaded whole down to every per-race tab and its
/// state dispatcher — no tab redeclares or lists the four signals.
#[derive(Props, Clone, Copy, PartialEq)]
pub struct RaceTabsProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}
