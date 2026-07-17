use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::customkeys::queries::unit_override_target_query::{
    UnitOverrideTargetQuery, UnitOverrideTargetRequest, UnitOverrideTargetView,
};
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::rc::Rc;
use warcraft_api::{WarcraftApi, WarcraftObjectId};
use warcraft_keybinds::{CustomKeys, GridSlotId, UnitSlotContainers};
use wasm_bindgen::JsCast;

use super::model::PagerCardModel;

#[derive(Clone, PartialEq)]
struct PagerCardIdentity {
    name: String,
    icon_url: Option<String>,
}

pub(super) struct PagerCardPresentation {
    pub(super) icon_url: Option<String>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) grid_count: usize,
    pub(super) active_grid_index: usize,
    pub(super) onscroll: EventHandler<ScrollEvent>,
    pub(super) override_target: UnitOverrideTargetView,
}

pub(super) fn use_pager_card(props: &PagerCardModel) -> PagerCardPresentation {
    let unit_id = props.unit_id;
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let loaded_keys = use_loaded_keys();
    // Memoized because the pager mounts several cards at once and re-renders them
    // on every scroll frame. Answering the query per render resolved slot
    // containers and rebuilt the inspector detail for each mounted card, which is
    // what made the wheel drag.
    let override_target_memo = use_memo(move || {
        let inspector_slot = *selected_slot.read();
        let inspector_from_research = *selected_from_research.read();
        let inspector_from_uprooted = *selected_from_uprooted.read();
        let keys_guard = loaded_keys.read();
        let custom_keys_ref: &Option<CustomKeys> = &keys_guard;
        let override_request = UnitOverrideTargetRequest {
            unit_id,
            selected_slot: inspector_slot,
            from_uprooted: inspector_from_uprooted,
            from_research: inspector_from_research,
        };
        let override_query = UnitOverrideTargetQuery::new(override_request);
        override_query.answer(custom_keys_ref)
    });
    let slot_data = use_memo(move || UnitSlotContainers::from(unit_id));
    let identity = use_memo(move || {
        let api = WarcraftApi::default();
        let unit_view = api.unit().get(unit_id);
        let display_name = unit_view
            .as_ref()
            .and_then(|unit| unit.name())
            .unwrap_or("(unnamed)")
            .to_string();
        let icon_url = unit_view
            .as_ref()
            .and_then(|unit| unit.icon())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        PagerCardIdentity {
            name: display_name,
            icon_url,
        }
    });

    let active_grid_index_signal = use_signal::<usize>(|| 0);
    let onscroll = use_hook(|| {
        let mut active_grid_index_writer = active_grid_index_signal;
        EventHandler::new(move |event: ScrollEvent| {
            let Some(web_event) = event.data().try_as_web_event() else {
                return;
            };
            let Some(scroll_target) = web_event.target() else {
                return;
            };
            let Ok(carousel_element) = scroll_target.dyn_into::<web_sys::Element>() else {
                return;
            };
            let carousel_client_width = carousel_element.client_width();
            if carousel_client_width <= 0 {
                return;
            }
            let carousel_scroll_left = carousel_element.scroll_left();
            let half_pane_width = carousel_client_width / 2;
            let raw_index = (carousel_scroll_left + half_pane_width) / carousel_client_width;
            let next_index = usize::try_from(raw_index).unwrap_or(0);
            if *active_grid_index_writer.peek() != next_index {
                active_grid_index_writer.set(next_index);
            }
        })
    });

    let slot_containers = slot_data.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();

    let resolved_identity = identity.read();
    let name = resolved_identity.name.clone();
    let icon_url = resolved_identity.icon_url.clone();

    let optional_grid_presence = [
        build_menu_slots.is_some(),
        uprooted_menu_slots.is_some(),
        research_menu_slots.is_some(),
    ];
    let additional_grid_count = optional_grid_presence
        .into_iter()
        .filter(|is_present| *is_present)
        .count();
    let grid_count = 1 + additional_grid_count;
    let active_grid_index = *active_grid_index_signal.read();

    let override_target = override_target_memo.read().clone();

    PagerCardPresentation {
        icon_url,
        name,
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        grid_count,
        active_grid_index,
        onscroll,
        override_target,
    }
}
