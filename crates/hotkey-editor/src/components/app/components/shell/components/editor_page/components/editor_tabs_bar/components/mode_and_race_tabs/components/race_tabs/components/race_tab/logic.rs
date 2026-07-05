use super::props::RaceTabProps;
use crate::services::focus::modality::FocusModality;
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

impl From<&RaceTabProps> for RaceTabChrome {
    fn from(props: &RaceTabProps) -> Self {
        let race = props.race;
        let is_active = props.is_active;
        let race_attribute = RaceLabels::data_attribute(race).to_string();
        let active_race = props.active_race;
        let unit_mode = props.unit_mode;
        let selected_unit_id = props.selected_unit_id;
        let selected_slot = props.selected_slot;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            apply_race(
                race,
                active_race,
                unit_mode,
                selected_unit_id,
                selected_slot,
            );
        });
        let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
            let key_value = event.key().to_string();
            if key_value == " " || key_value == "Enter" {
                event.prevent_default();
                apply_race(
                    race,
                    active_race,
                    unit_mode,
                    selected_unit_id,
                    selected_slot,
                );
                FocusModality::after_render(".unit-card[data-selected='true'], .unit-card");
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

fn apply_race(
    race: Race,
    mut active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) {
    active_race.set(race);
    let mode_value = *unit_mode.read();
    let next_id = UnitKindHelpers::default_unit_id_for(race, mode_value);
    selected_unit_id.set(next_id);
    selected_slot.set(None);
}
