use super::components::hero_level_backdrop::HeroLevelBackdropProps;
use super::components::hero_level_menu::HeroLevelMenuProps;
use super::components::hero_level_trigger::HeroLevelTriggerProps;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The hero-level dropdown's shaped state: whether it is open, and the trigger, menu,
/// and backdrop props. The picker OWNS its open signal (no open-flag is threaded in)
/// and SOURCES the selected level from editor context (no level signal is threaded in);
/// it resets itself closed whenever the selected unit changes.
pub(super) struct HeroLevelPickerView {
    pub(super) is_open: bool,
    pub(super) trigger: HeroLevelTriggerProps,
    pub(super) menu: HeroLevelMenuProps,
    pub(super) backdrop: HeroLevelBackdropProps,
}

pub(super) fn use_hero_level_picker() -> HeroLevelPickerView {
    let mut level_picker_open = use_signal::<bool>(|| false);
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id;
    use_effect(move || {
        let _ = selected_unit_id.read();
        level_picker_open.set(false);
    });
    let selected_hero_level = use_editor_state().selected_hero_level;
    let current_level = *selected_hero_level.read();
    let is_open = *level_picker_open.read();
    let number = current_level.to_string();
    let mut toggle_open = level_picker_open;
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !toggle_open();
        toggle_open.set(next);
    });
    let trigger = HeroLevelTriggerProps {
        number,
        is_open,
        onclick: toggle,
    };
    let menu = HeroLevelMenuProps {
        current_level,
        selected_hero_level,
        level_picker_open,
    };
    let mut close_open = level_picker_open;
    let dismiss = EventHandler::new(move |_event: MouseEvent| close_open.set(false));
    let backdrop = HeroLevelBackdropProps { onclick: dismiss };
    HeroLevelPickerView {
        is_open,
        trigger,
        menu,
        backdrop,
    }
}
