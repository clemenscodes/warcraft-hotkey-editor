use super::props::RaceTabProps;
use crate::services::focus::coordinator::{FocusCoordinator, FocusTarget};
use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};
use warcraft_database::{UnitKindHelpers, UnitMode};
use warcraft_keybinds::GridSlotId;

/// The race button's non-styling chrome: its `data-race` value, active flag, and
/// the pointer and keyboard activation handlers.
pub(super) struct RaceTabChrome {
    pub(super) race_attribute: String,
    pub(super) is_active: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

impl RaceTabChrome {
    pub(super) fn build(props: &RaceTabProps, focus: FocusCoordinator) -> Self {
        let race = props.race;
        let is_active = props.is_active;
        let race_attribute = RaceLabels::data_attribute(race).to_string();
        let active_race = props.active_race;
        let unit_mode = props.unit_mode;
        let selected_unit_id = props.selected_unit_id;
        let selected_slot = props.selected_slot;
        let selection = RaceSelection {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
        };
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            selection.apply(race);
        });
        let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
            let key_value = event.key().to_string();
            if key_value == " " || key_value == "Enter" {
                event.prevent_default();
                selection.apply(race);
                focus.request(FocusTarget::UnitCard);
            }
        });
        Self {
            race_attribute,
            is_active,
            onclick,
            onkeydown,
        }
    }
}

/// The editor selection a race click updates: the active race, plus the unit and
/// slot it resets. Grouping the signals makes "apply this race" a method rather than
/// a free function.
#[derive(Clone, Copy)]
struct RaceSelection {
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    selected_unit_id: Signal<Option<String>>,
    selected_slot: Signal<Option<GridSlotId>>,
}

impl RaceSelection {
    fn apply(self, race: Race) {
        let mut active_race = self.active_race;
        active_race.set(race);
        let mode_value = *self.unit_mode.read();
        let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
        let mut selected_unit_id = self.selected_unit_id;
        selected_unit_id.set(next_id);
        let mut selected_slot = self.selected_slot;
        selected_slot.set(None);
    }
}
