mod derived_stats;
pub mod grids;
pub mod unit_detail_header;
pub mod stats_panel;
pub mod tile_override_empty;
pub mod tile_override_panel;
pub mod unit_description;
pub mod unit_detail_empty;

use crate::components::tile_override::TileOverride;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use crate::model::icons::IconUrl;
use derived_stats::DerivedStats;
use dioxus::prelude::*;
use grids::UnitCommandGrids;
use unit_detail_header::UnitDetailHeader;
use stats_panel::UnitStatsPanel;
use std::collections::HashMap;
use std::rc::Rc;
use tile_override_empty::TileOverrideEmpty;
use tile_override_panel::TileOverridePanel;
use unit_description::UnitDescription;
use unit_detail_empty::UnitDetailEmpty;
use warcraft_api::{Race, WarcraftObjectMeta};
use warcraft_database::{ObjectLookup, WARCRAFT_DATABASE};
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::{CustomKeys, InspectorDetail, UnitCommandSlots};

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailPanelProps {
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
}

#[component]
pub fn UnitDetailPanel(props: UnitDetailPanelProps) -> Element {
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
        let unit_id_obj = WARCRAFT_DATABASE
            .by_id_and_key(unit_id_str)
            .map(|(id, _)| id)
            .unwrap_or_default();
        let command_card_slots: Rc<[GridSlotId]> = WARCRAFT_DATABASE
            .command_card(unit_id_obj)
            .filled_slots()
            .collect();
        let build_menu_slots: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .build_menu(unit_id_obj)
            .map(|card| card.filled_slots().collect());
        let uprooted_menu_slots: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .uprooted_menu(unit_id_obj)
            .map(|card| card.filled_slots().collect());
        let research_menu_slots: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .research_menu(unit_id_obj)
            .map(|card| card.filled_slots().collect());
        let train_upgrades = WARCRAFT_DATABASE.train_unit_upgrades(unit_id_obj);
        (
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
            train_upgrades,
        )
    });
    let unit_id_option = selected_unit_id.read().clone();
    let Some(unit_id) = unit_id_option else {
        return rsx! {
            UnitDetailEmpty { message: "Select a unit to view its command card." }
        };
    };
    let Some(unit_object) = ObjectLookup::by_id(&unit_id) else {
        return rsx! {
            UnitDetailEmpty { message: "Unit not found in database." }
        };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! {
            UnitDetailEmpty { message: "Selected object is not a unit." }
        };
    };
    let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
    let portrait_url = unit_object
        .icons()
        .first()
        .copied()
        .map(IconUrl::from_database_path)
        .map(|url| url.to_string());
    let slot_data_guard = slot_data_memo.read();
    let (
        command_card_slots_rc,
        build_menu_slots_rc,
        uprooted_menu_slots_rc,
        research_menu_slots_rc,
        train_upgrades,
    ) = slot_data_guard.clone();
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
        uprooted_menu_slots_rc
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else if inspector_from_research {
        research_menu_slots_rc
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else {
        let inspector_slot_id = inspector_slot
            .as_ref()
            .map(|slot| slot.as_str().to_string());
        let in_build_menu = inspector_slot_id.as_deref().is_some_and(|id_value| {
            build_menu_slots_rc.as_ref().is_some_and(|list| {
                list.iter()
                    .any(|candidate| candidate.as_str().eq_ignore_ascii_case(id_value))
            })
        });
        if in_build_menu {
            build_menu_slots_rc
                .clone()
                .unwrap_or_else(|| empty_slot_list.clone())
        } else {
            command_card_slots_rc.clone()
        }
    };
    let unit_description = unit_object.ubertip();
    let unit_combat = *unit_meta.combat();
    let hero_attributes_option = unit_meta.hero_attributes().copied();
    let unit_evasion_chance = DerivedStats::unit_evasion_chance(unit_meta);
    rsx! {
        section { class: "unit-detail",
            UnitDetailHeader {
                unit_name,
                unit_id: unit_id
                        .clone(),
                portrait_url,
                has_hero_attributes: hero_attributes_option.is_some(),
                selected_hero_level,
                level_picker_open,
            }
            UnitDescription { text: unit_description.unwrap_or_default() }
            UnitStatsPanel {
                combat: unit_combat,
                hero_attributes: hero_attributes_option,
                selected_hero_level,
                evasion_chance: unit_evasion_chance,
            }
            div { class: "unit-detail-body",
                div { class: "unit-detail-row",
                    UnitCommandGrids {
                        unit_id: unit_id.clone(),
                        race,
                        command_card_slots: command_card_slots_rc,
                        build_menu_slots: build_menu_slots_rc,
                        uprooted_menu_slots: uprooted_menu_slots_rc,
                        research_menu_slots: research_menu_slots_rc,
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
                    }
                    TileOverridePanel {
                        h3 { class: "grid-heading", "Hotkey override" }
                        if let Some(detail) = inspector_panel.clone() {
                            TileOverride {
                                detail,
                                loaded_keys,
                                grid_layout,
                                selected_from_research,
                                selected_from_uprooted,
                                tier_overrides,
                                dragging_slot,
                                drop_target_tile,
                                drag_follower,
                                active_container_slots: active_container_slots.clone(),
                                hotkey_assign_request,
                            }
                        } else {
                            TileOverrideEmpty { message: "Select a tile in the grid to override its hotkey." }
                        }
                    }
                }
            }
        }
    }
}
