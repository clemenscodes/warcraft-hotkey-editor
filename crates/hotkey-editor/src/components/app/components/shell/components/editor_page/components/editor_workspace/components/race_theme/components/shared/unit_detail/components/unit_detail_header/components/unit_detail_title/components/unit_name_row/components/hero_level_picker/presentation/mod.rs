use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

pub(super) struct HeroLevelPickerView {
    pub(super) is_open: bool,
    pub(super) level_number: String,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) level_picker_open: Signal<bool>,
    pub(super) dismiss: EventHandler<MouseEvent>,
}

pub(super) fn use_hero_level_picker() -> HeroLevelPickerView {
    let mut level_picker_open = use_signal::<bool>(|| false);
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    use_effect(move || {
        let _ = selected_unit_id.read();
        level_picker_open.set(false);
    });
    let selected_hero_level = use_editor_state().selected_hero_level();
    let current_level = *selected_hero_level.read();
    let is_open = *level_picker_open.read();
    let level_number = current_level.to_string();
    let mut toggle_open = level_picker_open;
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !toggle_open();
        toggle_open.set(next);
    });
    let mut close_open = level_picker_open;
    let dismiss = EventHandler::new(move |_event: MouseEvent| close_open.set(false));
    HeroLevelPickerView {
        is_open,
        level_number,
        toggle,
        level_picker_open,
        dismiss,
    }
}
