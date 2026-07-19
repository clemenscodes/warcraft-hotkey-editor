use super::model::UnitDetailInputs;
use super::state::{UnitDetailModel, UnitDetailView};
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::customkeys::queries::unit_override_target_query::{
    UnitOverrideTargetQuery, UnitOverrideTargetRequest,
};
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftApi;
use warcraft_api::{Evasion, HeroAttributes, UnitCombat, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_keybinds::{CustomKeys, GridSlotId, UnitSlotContainers};

#[derive(Clone, PartialEq)]
pub(crate) struct UnitCommandGridSlots {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
}

#[derive(Clone, PartialEq)]
pub(super) struct ResolvedUnit {
    pub(super) unit_name: &'static str,
    pub(super) portrait_url: Option<String>,
    pub(super) description_text: String,
    pub(super) combat: UnitCombat,
    pub(super) hero_attributes: Option<HeroAttributes>,
    pub(super) evasion: Evasion,
}

impl TryFrom<WarcraftObjectId> for ResolvedUnit {
    type Error = &'static str;

    fn try_from(unit_id: WarcraftObjectId) -> Result<Self, Self::Error> {
        let api = WarcraftApi::default();
        let Some(unit_object) = api.object(unit_id) else {
            return Err("Unit not found in database.");
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Err("Selected object is not a unit.");
        };
        let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
        let portrait_url = unit_object
            .icons()
            .first()
            .copied()
            .map(IconUrl::from_database_path)
            .map(|url| url.to_string());
        let description_text = unit_object.ubertip().unwrap_or_default().to_string();
        let combat = *unit_meta.combat();
        let hero_attributes = unit_meta.hero_attributes().copied();
        let evasion = api.unit().evasion(unit_id);
        Ok(Self {
            unit_name,
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        })
    }
}

fn use_hero_level_reset(selected_unit_id: Signal<Option<WarcraftObjectId>>) {
    let mut selected_hero_level = use_editor_state().selected_hero_level();
    use_effect(move || {
        let _ = selected_unit_id.read();
        selected_hero_level.set(1);
    });
}

pub(super) fn use_unit_detail_panel() -> UnitDetailView {
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_unit = editor.selected_unit();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let loaded_keys = use_loaded_keys();
    use_hero_level_reset(selected_unit_id);
    let slot_data_memo = use_memo(move || {
        let unit_id_option = *selected_unit_id.read();
        let unit_id = unit_id_option.unwrap_or_default();
        UnitSlotContainers::from(unit_id)
    });
    let unit_id_option = *selected_unit_id.read();
    let Some(unit_id) = unit_id_option else {
        return UnitDetailView::Empty("Select a unit to view its command card.");
    };
    let resolved_unit = match ResolvedUnit::try_from(unit_id) {
        Ok(resolved) => resolved,
        Err(message) => return UnitDetailView::Empty(message),
    };
    let slot_containers = slot_data_memo.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();
    // Only a selection made on this unit fills its override, so a stale selection
    // from another unit (say after a layout switch) does not leak in.
    let inspector_slot = if *selected_unit.read() == Some(unit_id) {
        *selected_slot.read()
    } else {
        None
    };
    let inspector_from_uprooted = *selected_from_uprooted.read();
    let inspector_from_research = *selected_from_research.read();
    let keys_guard = loaded_keys.read();
    let custom_keys_ref: &Option<CustomKeys> = &keys_guard;
    let override_request = UnitOverrideTargetRequest {
        unit_id,
        selected_slot: inspector_slot,
        from_uprooted: inspector_from_uprooted,
        from_research: inspector_from_research,
    };
    let override_query = UnitOverrideTargetQuery::new(override_request);
    let override_target = override_query.answer(custom_keys_ref);
    drop(keys_guard);
    let inputs = UnitDetailInputs {
        unit_id,
        resolved_unit,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        override_target,
    };
    let model = UnitDetailModel::from(inputs);
    let boxed_model = Box::new(model);
    UnitDetailView::Loaded(boxed_model)
}
