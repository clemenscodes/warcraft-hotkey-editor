use super::components::unit_card_surface::UnitCardSurfaceProps;
use super::props::UnitCardProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::unit_kind_data_attr;
use crate::services::editor_state::context::use_editor_state;
use crate::services::focus::context::use_focus_coordinator;
use crate::services::focus::coordinator::FocusTarget;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::rc::Rc;

/// The card's shaped view: the kind data attribute the wrapper's carousel filter reads,
/// and the finished button surface (its selected look, select handlers, and mount
/// registration).
pub(super) struct UnitCardModel {
    pub(super) kind_attr: &'static str,
    pub(super) surface: UnitCardSurfaceProps,
}

/// Reads the selection from context: the card is selected when it is the selected unit,
/// and selecting it (click or Space/Enter) sets it as the selected unit, clears any
/// selected slot, and switches the active category to the card's kind. It also
/// registers itself as the unit-card focus target exactly while it is selected.
pub(super) fn use_unit_card(props: &UnitCardProps) -> UnitCardModel {
    let unit_id = props.unit_id;
    let unit_kind = props.unit_kind;
    let kind_attr = unit_kind_data_attr(unit_kind);
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let mut selected_unit_id = navigation.selected_unit_id();
    let mut selected_slot = editor.selected_slot();
    let mut active_category = editor.active_category();
    let is_selected = *selected_unit_id.read() == Some(unit_id);
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    use_effect(move || {
        if *selected_unit_id.read() == Some(unit_id) {
            let handle = mounted_handle.read().clone();
            focus.set_unit_card_handle(handle);
        }
    });
    let on_mounted = EventHandler::new(move |event: Event<MountedData>| {
        mounted_handle.set(Some(event.data()));
    });
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        selected_unit_id.set(Some(unit_id));
        selected_slot.set(None);
        active_category.set(unit_kind);
    });
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            selected_unit_id.set(Some(unit_id));
            selected_slot.set(None);
            active_category.set(unit_kind);
            focus.request(FocusTarget::UnitCard);
        }
    });
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let surface = UnitCardSurfaceProps {
        icon_path,
        display_name,
        unit_id,
        is_selected,
        onclick: on_click,
        onkeydown: on_keydown,
        onmounted: on_mounted,
    };
    UnitCardModel { kind_attr, surface }
}
