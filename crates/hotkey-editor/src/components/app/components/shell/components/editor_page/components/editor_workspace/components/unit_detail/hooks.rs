use super::components::unit_description::UnitDescriptionProps;
use super::components::unit_detail_body::UnitDetailBodyProps;
use super::components::unit_detail_body::components::unit_detail_row::UnitDetailRowProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGridsProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::UnitTileOverrideProps;
use super::components::unit_detail_header::UnitDetailHeaderProps;
use super::components::unit_stats_panel::UnitStatsPanelProps;
use super::logic::{ActiveContainer, InspectorPanel, ResolvedUnit};
use super::props::UnitDetailPanelProps;
use super::state::{UnitDetailModel, UnitDetailView};
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_keybinds::UnitSlotContainers;

/// The hero-level picker state: the currently-chosen level, reset to its default
/// whenever the selected unit changes. (The picker owns its own open state.)
pub(super) struct HeroLevelState {
    pub(super) selected_hero_level: Signal<u32>,
}

fn use_hero_level_state(selected_unit_id: Signal<Option<String>>) -> HeroLevelState {
    let mut selected_hero_level = use_editor_state().selected_hero_level;
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
/// them and assembles the child props.
pub(super) fn use_unit_detail_panel(props: &UnitDetailPanelProps) -> UnitDetailView {
    let race = *props.active_race.read();
    let selected_unit_id = props.selected_unit_id;
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot;
    let selected_from_research = editor.selected_from_research;
    let selected_from_uprooted = editor.selected_from_uprooted;
    let loaded_keys = use_loaded_keys();
    let hero_level = use_hero_level_state(selected_unit_id);
    let slot_data_memo = use_memo(move || {
        let unit_id_option = selected_unit_id.read().clone();
        let unit_id_str = unit_id_option.as_deref().unwrap_or("");
        UnitSlotContainers::resolve(unit_id_str)
    });
    let unit_id_option = selected_unit_id.read().clone();
    let Some(unit_id) = unit_id_option else {
        return UnitDetailView::Empty("Select a unit to view its command card.");
    };
    let resolved_unit = match ResolvedUnit::resolve(&unit_id) {
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
    let inspector_panel = InspectorPanel::resolve(
        &inspector_slot,
        &keys_guard,
        &unit_id,
        inspector_from_uprooted,
        inspector_from_research,
        train_upgrades,
    );
    drop(keys_guard);
    let active_container = ActiveContainer::resolve(
        &slot_containers,
        &inspector_slot,
        inspector_from_uprooted,
        inspector_from_research,
    );
    let active_container_slots = active_container.slots;
    let header = UnitDetailHeaderProps {
        unit_name: resolved_unit.unit_name,
        unit_id: unit_id.clone(),
        portrait_url: resolved_unit.portrait_url,
        has_hero_attributes: resolved_unit.hero_attributes.is_some(),
    };
    let description = UnitDescriptionProps {
        text: resolved_unit.description_text,
    };
    let stats = UnitStatsPanelProps {
        combat: resolved_unit.combat,
        hero_attributes: resolved_unit.hero_attributes,
        selected_hero_level: hero_level.selected_hero_level,
        evasion: resolved_unit.evasion,
    };
    let grids = UnitCommandGridsProps {
        unit_id: unit_id.clone(),
        race,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    };
    let tile_override = UnitTileOverrideProps {
        detail: inspector_panel.detail,
        active_container_slots,
    };
    let row = UnitDetailRowProps {
        grids,
        tile_override,
    };
    let body = UnitDetailBodyProps { row };
    let model = UnitDetailModel {
        header,
        description,
        stats,
        body,
    };
    UnitDetailView::Loaded(Box::new(model))
}
