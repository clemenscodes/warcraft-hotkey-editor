mod state;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};

use crate::model::grid::GridSlotId;
use crate::model::icons::IconUrl;
use crate::services::focus::modality::FocusModality;

use super::unit_kind_data_attr;
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
    let kind_attr = unit_kind_data_attr(unit_kind);
    let unit_id_for_click = unit_id.clone();
    let unit_id_for_keydown = unit_id.clone();
    let unit_id_label = unit_id.clone();
    let kind_for_click = unit_kind;
    let kind_for_keydown = unit_kind;

    rsx! {
        button {
            class: "{classes.button_class()}",
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
                    FocusModality::after_render(".unit-card.selected, .unit-card");
                }
            },
            if let Some(source) = icon_path {
                img {
                    class: "w-20 h-20 border border-warcraft-blue rounded-[3px] shrink-0 object-cover bg-[rgba(20,35,60,0.7)] text-transparent",
                    src: "{source}",
                    alt: "{display_name}",
                    loading: "lazy",
                    decoding: "async",
                }
            } else {
                div { class: "w-20 h-20 border border-warcraft-blue rounded-[3px] shrink-0 bg-[rgba(20,35,60,0.7)]" }
            }
            div { class: "flex flex-col gap-[0.45rem] min-w-0 flex-1",
                span {
                    class: "text-[1.05rem] leading-[1.25] pb-[0.1rem] overflow-hidden text-ellipsis whitespace-nowrap min-w-0 min-[1900px]:text-[1.35rem]",
                    "{display_name}"
                }
                code { class: "{classes.id_class()}", "{unit_id_label}" }
            }
        }
    }
}
