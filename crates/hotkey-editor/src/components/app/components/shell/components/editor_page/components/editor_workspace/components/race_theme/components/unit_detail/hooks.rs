use super::logic::{
    ActiveContainer, ActiveContainerInputs, InspectorPanel, InspectorPanelInputs, ResolvedUnit,
};
use super::props::UnitDetailInputs;
use super::state::{UnitDetailModel, UnitDetailView};
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{CustomKeys, UnitSlotContainers};

/// The hero-level picker state: the currently-chosen level, reset to its default
/// whenever the selected unit changes. (The picker owns its own open state.)
pub(super) struct HeroLevelState {
    pub(super) selected_hero_level: Signal<u32>,
}

fn use_hero_level_state(selected_unit_id: Signal<Option<WarcraftObjectId>>) -> HeroLevelState {
    let mut selected_hero_level = use_editor_state().selected_hero_level();
    use_effect(move || {
        let _ = selected_unit_id.read();
        selected_hero_level.set(1);
    });
    HeroLevelState {
        selected_hero_level,
    }
}

/// Resolves the selected unit and shapes every child's props. The domain work is
/// grouped into the [`ResolvedUnit`], [`InspectorPanel`], and [`ActiveContainer`]
/// derivations plus the memoized [`UnitSlotContainers`]; this hook only orchestrates
/// them, gathers the [`UnitDetailInputs`], and lets the props tree derive itself.
pub(super) fn use_unit_detail_panel() -> UnitDetailView {
    let navigation = use_view_navigation();
    let race = *navigation.active_race().read();
    let selected_unit_id = navigation.selected_unit_id();
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let loaded_keys = use_loaded_keys();
    let hero_level = use_hero_level_state(selected_unit_id);
    let slot_data_memo = use_memo(move || {
        let unit_id_option = *selected_unit_id.read();
        let unit_id = unit_id_option.unwrap_or_default();
        UnitSlotContainers::resolve(unit_id)
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
    let inspector_slot = *selected_slot.read();
    let inspector_from_uprooted = *selected_from_uprooted.read();
    let inspector_from_research = *selected_from_research.read();
    let keys_guard = loaded_keys.read();
    let train_upgrades = slot_containers.train_upgrades();
    let custom_keys_ref: &Option<CustomKeys> = &keys_guard;
    let inspector_inputs = InspectorPanelInputs {
        inspector_slot: &inspector_slot,
        custom_keys: custom_keys_ref,
        host_unit_id: unit_id,
        from_uprooted: inspector_from_uprooted,
        from_research: inspector_from_research,
        train_upgrades,
    };
    let inspector_panel = InspectorPanel::from(inspector_inputs);
    drop(keys_guard);
    let containers_ref: &UnitSlotContainers = &slot_containers;
    let active_container_inputs = ActiveContainerInputs {
        containers: containers_ref,
        inspector_slot: &inspector_slot,
        from_uprooted: inspector_from_uprooted,
        from_research: inspector_from_research,
    };
    let active_container = ActiveContainer::from(active_container_inputs);
    let active_container_slots = active_container.slots;
    let selected_hero_level = hero_level.selected_hero_level;
    let detail = inspector_panel.detail;
    let inputs = UnitDetailInputs {
        race,
        unit_id,
        resolved_unit,
        selected_hero_level,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        detail,
        active_container_slots,
    };
    let model = UnitDetailModel::from(inputs);
    let boxed_model = Box::new(model);
    UnitDetailView::Loaded(boxed_model)
}
