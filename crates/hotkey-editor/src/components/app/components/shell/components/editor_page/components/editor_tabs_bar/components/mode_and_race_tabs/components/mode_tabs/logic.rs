use super::components::mode_tab::ModeTabProps;
use super::props::ModeTabsProps;
use crate::services::focus::coordinator::{FocusCoordinator, FocusTarget};
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::{UnitKindHelpers, UnitMode};
use warcraft_keybinds::GridSlotId;

/// The Melee and Campaign buttons, each finished with its label, active flag, and
/// event handlers.
pub(super) struct ModeTabPair {
    pub(super) melee: ModeTabProps,
    pub(super) campaign: ModeTabProps,
}

impl ModeTabPair {
    pub(super) fn build(props: &ModeTabsProps, focus: FocusCoordinator) -> Self {
        let melee = mode_tab(props, focus, UnitMode::Melee, "Melee");
        let campaign = mode_tab(props, focus, UnitMode::Campaign, "Campaign");
        Self { melee, campaign }
    }
}

/// Builds one mode button: selecting it switches the mode, picks that mode's
/// default unit for the current race, and clears the slot selection. The keyboard
/// activation additionally moves focus onto the race tabs.
fn mode_tab(
    props: &ModeTabsProps,
    focus: FocusCoordinator,
    mode: UnitMode,
    label: &'static str,
) -> ModeTabProps {
    let unit_mode = props.unit_mode;
    let active_race = props.active_race;
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let active = *unit_mode.read() == mode;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        apply_mode(
            mode,
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
            apply_mode(
                mode,
                active_race,
                unit_mode,
                selected_unit_id,
                selected_slot,
            );
            focus.request(FocusTarget::RaceTabs);
        }
    });
    ModeTabProps {
        label,
        active,
        onclick,
        onkeydown,
    }
}

fn apply_mode(
    mode: UnitMode,
    active_race: Signal<Race>,
    mut unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
) {
    unit_mode.set(mode);
    let race = *active_race.read();
    let next_id = UnitKindHelpers::default_unit_id_for(race, mode);
    selected_unit_id.set(next_id);
    selected_slot.set(None);
}
