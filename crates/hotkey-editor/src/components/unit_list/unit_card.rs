use dioxus::prelude::*;
use warcraft_api::UnitKind;

use crate::focus::modality::FocusModality;
use crate::grid_slot::GridSlotId;
use crate::icons::IconUrl;

use super::unit_kind_data_attr;

#[component]
pub(super) fn UnitCard(
    unit_id: String,
    display_name: String,
    icon_path: Option<IconUrl>,
    unit_kind: UnitKind,
    is_selected: bool,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut active_category: Signal<UnitKind>,
) -> Element {
    let class_name = if is_selected {
        "unit-card selected"
    } else {
        "unit-card"
    };
    let unit_id_for_click = unit_id.clone();
    let unit_id_for_keydown = unit_id.clone();
    let unit_id_label = unit_id.clone();
    let kind_for_click = unit_kind;
    let kind_for_keydown = unit_kind;
    let kind_attr = unit_kind_data_attr(unit_kind);

    rsx! {
        button {
            class: "{class_name}",
            "data-unit-kind": "{kind_attr}",
            onclick: move |_| {
                selected_unit_id.set(Some(unit_id_for_click.clone()));
                selected_slot.set(None);
                active_category.set(kind_for_click);
            },
            onkeydown: move |event| {
                let key_value = event.data().key().to_string();
                if key_value == " " || key_value == "Enter" {
                    event.prevent_default();
                    selected_unit_id.set(Some(unit_id_for_keydown.clone()));
                    selected_slot.set(None);
                    active_category.set(kind_for_keydown);
                    FocusModality::after_render(".grid-tile.has-ability.selected, .grid-tile.has-ability");
                }
            },
            if let Some(source) = icon_path {
                img {
                    class: "unit-card-icon",
                    src: "{source}",
                    alt: "{display_name}",
                    loading: "lazy",
                    decoding: "async",
                }
            } else {
                div { class: "unit-card-icon" }
            }
            div { class: "unit-card-text",
                span { class: "unit-card-name", "{display_name}" }
                code { class: "unit-card-id", "{unit_id_label}" }
            }
        }
    }
}
