mod icon;
mod info;
mod state;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};

use crate::model::grid::GridSlotId;
use crate::model::icons::IconUrl;
use crate::services::focus::modality::FocusModality;

use super::unit_kind_data_attr;
use icon::UnitCardIcon;
use info::UnitCardInfo;
use state::UnitCardClasses;

#[derive(Props, Clone, PartialEq)]
pub(super) struct UnitCardProps {
    pub(super) unit_id: String,
    pub(super) display_name: String,
    pub(super) icon_path: Option<IconUrl>,
    pub(super) unit_kind: UnitKind,
    pub(super) race: Race,
    pub(super) is_selected: bool,
    pub(super) selected_unit_id: Signal<Option<String>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

#[component]
pub(super) fn UnitCard(props: UnitCardProps) -> Element {
    let unit_id = props.unit_id;
    let display_name = props.display_name;
    let icon_path = props.icon_path;
    let unit_kind = props.unit_kind;
    let race = props.race;
    let is_selected = props.is_selected;
    let mut selected_unit_id = props.selected_unit_id;
    let mut selected_slot = props.selected_slot;
    let mut active_category = props.active_category;
    let classes = UnitCardClasses::compute(is_selected, race);
    let button_class = classes.button_class();
    let id_class = classes.id_class();
    let kind_attr = unit_kind_data_attr(unit_kind);
    let unit_id_for_click = unit_id.clone();
    let unit_id_for_keydown = unit_id.clone();
    let display_name_for_icon = display_name.clone();
    let kind_for_click = unit_kind;
    let kind_for_keydown = unit_kind;

    let handle_click = move |_| {
        selected_unit_id.set(Some(unit_id_for_click.clone()));
        selected_slot.set(None);
        active_category.set(kind_for_click);
    };
    let handle_keydown = move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            selected_unit_id.set(Some(unit_id_for_keydown.clone()));
            selected_slot.set(None);
            active_category.set(kind_for_keydown);
            FocusModality::after_render(".unit-card.selected, .unit-card");
        }
    };

    rsx! {
        button {
            class: button_class,
            "data-unit-kind": kind_attr,
            onclick: handle_click,
            onkeydown: handle_keydown,
            UnitCardIcon { icon_path, display_name: display_name_for_icon }
            UnitCardInfo { display_name, unit_id, id_class }
        }
    }
}
