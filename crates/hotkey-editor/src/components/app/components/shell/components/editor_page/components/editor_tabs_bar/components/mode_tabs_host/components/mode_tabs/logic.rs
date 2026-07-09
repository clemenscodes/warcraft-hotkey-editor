use super::components::mode_tab::ModeTabProps;
use super::props::ModeTabsProps;
use crate::services::focus::coordinator::{FocusCoordinator, FocusTarget};
use dioxus::prelude::*;
use warcraft_database::UnitMode;

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

/// Builds one mode button: selecting it dispatches `on_select(mode)`; the keyboard
/// activation additionally moves focus onto the race tabs. The mode-change cascade
/// (default unit, slot reset) lives behind the handler, in the navigation service.
fn mode_tab(
    props: &ModeTabsProps,
    focus: FocusCoordinator,
    mode: UnitMode,
    label: &'static str,
) -> ModeTabProps {
    let unit_mode = props.unit_mode;
    let on_select = props.on_select;
    let active = *unit_mode.read() == mode;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        on_select.call(mode);
    });
    let onkeydown = EventHandler::new(move |event: KeyboardEvent| {
        let key = event.key();
        let key_value = key.to_string();
        let is_space = key_value == " ";
        let is_enter = key_value == "Enter";
        if is_space || is_enter {
            event.prevent_default();
            on_select.call(mode);
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
