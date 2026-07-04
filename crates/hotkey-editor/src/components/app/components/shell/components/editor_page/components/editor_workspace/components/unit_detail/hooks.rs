use super::props::UnitDetailPanelProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGridsProps;
use super::components::unit_description::UnitDescriptionProps;
use super::components::unit_detail_body::UnitDetailBodyProps;
use super::components::unit_detail_header::UnitDetailHeaderProps;
use super::components::unit_detail_body::components::unit_detail_row::UnitDetailRowProps;
use super::components::unit_stats_panel::UnitStatsPanelProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::UnitTileOverrideProps;
use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectMeta;
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{Evasion, GridSlotId, InspectorDetail, UnitSlotContainers};

/// The panel's shaped view: either an empty-state message, or the fully-built child
/// props for the loaded unit.
pub(super) enum UnitDetailView {
    Empty(&'static str),
    Loaded(Box<UnitDetailModel>),
}

/// Every child's finished props for a loaded unit.
pub(super) struct UnitDetailModel {
    pub(super) header: UnitDetailHeaderProps,
    pub(super) description: UnitDescriptionProps,
    pub(super) stats: UnitStatsPanelProps,
    pub(super) body: UnitDetailBodyProps,
}

/// Resolves the selected unit and shapes every child's props. All the domain work
/// (database lookups, inspector resolution, active-container selection) lives here.
pub(super) fn use_unit_detail_panel(props: &UnitDetailPanelProps) -> UnitDetailView {
    let race = *props.active_race.read();
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let selected_from_research = props.selected_from_research;
    let selected_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let dragging_slot = props.dragging_slot;
    let drop_target_tile = props.drop_target_tile;
    let drag_follower = props.drag_follower;
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
    let hotkey_assign_request = props.hotkey_assign_request;
    let mut selected_hero_level = use_signal::<u32>(|| 1);
    let mut level_picker_open = use_signal::<bool>(|| false);
    use_effect(move || {
        let _ = selected_unit_id.read();
        selected_hero_level.set(1);
        level_picker_open.set(false);
    });
    let slot_data_memo = use_memo(move || {
        let unit_id_option = selected_unit_id.read().clone();
        let unit_id_str = unit_id_option.as_deref().unwrap_or("");
        UnitSlotContainers::resolve(unit_id_str)
    });
    let unit_id_option = selected_unit_id.read().clone();
    let Some(unit_id) = unit_id_option else {
        return UnitDetailView::Empty("Select a unit to view its command card.");
    };
    let Some(unit_object) = ObjectLookup::by_id(&unit_id) else {
        return UnitDetailView::Empty("Unit not found in database.");
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return UnitDetailView::Empty("Selected object is not a unit.");
    };
    let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
    let portrait_url = unit_object
        .icons()
        .first()
        .copied()
        .map(IconUrl::from_database_path)
        .map(|url| url.to_string());
    let slot_containers = slot_data_memo.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();
    let train_upgrades = slot_containers.train_upgrades().clone();
    let inspector_slot = *selected_slot.read();
    let inspector_from_uprooted = *selected_from_uprooted.read();
    let inspector_from_research = *selected_from_research.read();
    let inspector_panel = inspector_slot.as_ref().map(|slot| {
        let upgrade_id = if let GridSlotId::Ability(id) = slot {
            train_upgrades.get(&id.object_id()).copied()
        } else {
            None
        };
        InspectorDetail::build(
            slot,
            &loaded_keys.read(),
            &unit_id,
            inspector_from_uprooted,
            inspector_from_research,
            upgrade_id,
        )
    });
    let empty_slot_list: Rc<[GridSlotId]> = Rc::from(Vec::<GridSlotId>::new());
    let active_container_slots: Rc<[GridSlotId]> = if inspector_from_uprooted {
        uprooted_menu_slots
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else if inspector_from_research {
        research_menu_slots
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else {
        let inspector_slot_id = inspector_slot
            .as_ref()
            .map(|slot| slot.as_str().to_string());
        let in_build_menu = inspector_slot_id.as_deref().is_some_and(|id_value| {
            build_menu_slots.as_ref().is_some_and(|list| {
                list.iter()
                    .any(|candidate| candidate.as_str().eq_ignore_ascii_case(id_value))
            })
        });
        if in_build_menu {
            build_menu_slots
                .clone()
                .unwrap_or_else(|| empty_slot_list.clone())
        } else {
            command_card_slots.clone()
        }
    };
    let unit_description = unit_object.ubertip();
    let unit_combat = *unit_meta.combat();
    let hero_attributes_option = unit_meta.hero_attributes().copied();
    let unit_evasion = Evasion::resolve(unit_meta);
    let header = UnitDetailHeaderProps {
        unit_name,
        unit_id: unit_id.clone(),
        portrait_url,
        has_hero_attributes: hero_attributes_option.is_some(),
        selected_hero_level,
        level_picker_open,
    };
    let description = UnitDescriptionProps {
        text: unit_description.unwrap_or_default().to_string(),
    };
    let stats = UnitStatsPanelProps {
        combat: unit_combat,
        hero_attributes: hero_attributes_option,
        selected_hero_level,
        evasion: unit_evasion,
    };
    let grids = UnitCommandGridsProps {
        unit_id: unit_id.clone(),
        race,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        loaded_keys,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        grid_layout,
        update_hotkeys_on_move,
        hotkey_assign_request,
    };
    let tile_override = UnitTileOverrideProps {
        detail: inspector_panel,
        loaded_keys,
        grid_layout,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        active_container_slots,
        hotkey_assign_request,
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
