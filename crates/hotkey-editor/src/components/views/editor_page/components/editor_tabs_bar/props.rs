use crate::components::views::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::ModeAndRaceTabsProps;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::GridSlotId;

/// The editor's top bar input: the mode and race navigation state it hands to the
/// mode-and-race tabs.
#[derive(Props, Clone, PartialEq)]
pub struct EditorTabsBarProps {
    pub unit_mode: Signal<UnitMode>,
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
}

impl From<&EditorTabsBarProps> for ModeAndRaceTabsProps {
    fn from(props: &EditorTabsBarProps) -> Self {
        Self {
            unit_mode: props.unit_mode,
            active_race: props.active_race,
            selected_unit_id: props.selected_unit_id,
            selected_slot: props.selected_slot,
        }
    }
}
