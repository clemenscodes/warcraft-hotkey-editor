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

#[component]
pub(super) fn UnitCard(
    unit_id: String,
    display_name: String,
    icon_path: Option<IconUrl>,
    unit_kind: UnitKind,
    race: Race,
    is_selected: bool,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut active_category: Signal<UnitKind>,
) -> Element {
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
