use super::props::UnitCardProps;
use super::style;
use crate::services::focus::modality::FocusModality;
use dioxus::prelude::*;

/// The card's shaped view: its full class string (base plus the mobile carousel
/// filter for its kind), the kind data attribute, and the select handlers.
pub(super) struct UnitCardModel {
    pub(super) class: crate::styling::ClassList,
    pub(super) kind_attr: &'static str,
    pub(super) on_click: EventHandler<MouseEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
}

/// Selecting a card (click or Space/Enter) sets it as the selected unit, clears any
/// selected slot, and switches the active category to the card's kind.
pub(super) fn use_unit_card(props: &UnitCardProps) -> UnitCardModel {
    let unit_kind = props.unit_kind;
    let class = style::class(props.race);
    let kind_attr = crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_list::unit_kind_data_attr(unit_kind);
    let mut selected_unit_id = props.selected_unit_id;
    let mut selected_slot = props.selected_slot;
    let mut active_category = props.active_category;
    let unit_id_for_click = props.unit_id.clone();
    let unit_id_for_keydown = props.unit_id.clone();
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        selected_unit_id.set(Some(unit_id_for_click.clone()));
        selected_slot.set(None);
        active_category.set(unit_kind);
    });
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            selected_unit_id.set(Some(unit_id_for_keydown.clone()));
            selected_slot.set(None);
            active_category.set(unit_kind);
            FocusModality::after_render(".unit-card.selected, .unit-card");
        }
    });
    UnitCardModel {
        class,
        kind_attr,
        on_click,
        on_keydown,
    }
}
