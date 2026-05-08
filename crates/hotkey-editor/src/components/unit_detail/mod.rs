mod grids;
mod header;
mod leveled_stats;
mod stat_icon;
mod stats_panel;

use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectMeta;
use warcraft_database::{ObjectLookup, WARCRAFT_DATABASE};
use warcraft_keybinds::{CustomKeys, InspectorDetail, UnitCommandSlots};

use crate::components::tile_override::TileOverridePanel;
use crate::model::grid::GridLayout;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::model::icons::IconUrl;

use grids::UnitCommandGrids;
use header::UnitDetailHeader;
use stats_panel::UnitStatsPanel;

#[component]
pub(crate) fn UnitDetailPanel(
    selected_unit_id: Signal<Option<String>>,
    selected_slot: Signal<Option<GridSlotId>>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    tier_overrides: Signal<HashMap<String, usize>>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    loaded_keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
) -> Element {
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
            section { class: "unit-detail empty",
                "Select a unit to view its command card."
            }
        };
    };

    let Some(unit_object) = ObjectLookup::by_id(&unit_id) else {
        return rsx! {
            section { class: "unit-detail empty", "Unit not found in database." }
        };
    };

    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! {
            section { class: "unit-detail empty", "Selected object is not a unit." }
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

    rsx! {
        section { class: "unit-detail",
            UnitDetailHeader {
                unit_name,
                unit_id: unit_id.clone(),
                portrait_url,
                has_hero_attributes: hero_attributes_option.is_some(),
                selected_hero_level,
                level_picker_open,
            }
            p { class: "unit-description", "{unit_description.unwrap_or_default()}" }
            UnitStatsPanel {
                combat: unit_combat,
                hero_attributes: hero_attributes_option,
                selected_hero_level,
            }
            div { class: "unit-detail-body",
                div { class: "unit-detail-row",
                    UnitCommandGrids {
                        unit_id: unit_id.clone(),
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
                        drop_target_cell,
                        drag_follower,
                        grid_layout,
                    }
                    aside { class: "tile-override-panel",
                        h3 { class: "command-section-heading", "Hotkey override" }
                        if let Some(detail) = inspector_panel.clone() {
                            TileOverridePanel {
                                detail,
                                loaded_keys,
                                grid_layout,
                                selected_from_research,
                                selected_from_uprooted,
                                tier_overrides,
                                dragging_slot,
                                drop_target_cell,
                                drag_follower,
                                active_container_slots: active_container_slots.clone(),
                            }
                        } else {
                            div { class: "tile-override-empty",
                                p { "Select a tile in the grid to override its hotkey." }
                            }
                        }
                    }
                }
            }
        }
    }
}
