use super::fixtures;
use dioxus::prelude::*;
use dioxus_gallery::Story;
use super::keys_mount::CustomKeysMount;
use super::toast_mount::ToastMount;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::ability_description::AbilityDescription;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::alt_state_section::AltStateSection;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::OverrideKey;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::upgrade_section::UpgradeSection;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::upgrade_tier::UpgradeTier;

use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::TileOverride;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_alt_picker::components::alt_position_picker::AltPositionPicker;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_upgrade_picker::components::upgrade_position_picker::UpgradePositionPicker;

use super::editor_mount::EditorMount;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::{CustomKeys, GridSlotId, InspectorDetail, UnitCommandSlots};

pub fn stories() -> Vec<Story> {
    vec![
        Story::single(
            "Tile override",
            "AltPositionPicker",
            alt_position_picker_open,
        ),
        Story::single(
            "Tile override",
            "UpgradePositionPicker",
            upgrade_position_picker_open,
        ),
        Story::single(
            "Tile override",
            "TileOverride",
            tile_override_panel_archmage_blizzard,
        ),
        Story::single(
            "Tile override",
            "AbilityDescription",
            ability_description_two_lines,
        ),
        Story::new(
            "Tile override",
            "OverrideKey",
            "Idle",
            override_key_field_idle,
        ),
        Story::new(
            "Tile override",
            "OverrideKey",
            "Editing",
            override_key_field_editing,
        ),
        Story::new(
            "Tile override",
            "OverrideKey",
            "Special token",
            override_key_field_special,
        ),
        Story::single(
            "Tile override",
            "UpgradeTier",
            upgrade_tier_selector_default,
        ),
        Story::single("Tile override", "UpgradeSection", upgrade_section_idle),
        Story::new(
            "Tile override",
            "AltStateSection",
            "With controls",
            alt_state_section_with_controls,
        ),
        Story::new(
            "Tile override",
            "AltStateSection",
            "No controls",
            alt_state_section_no_controls,
        ),
    ]
}

fn ability_description_two_lines() -> Element {
    let description_lines = vec![
        "Increases attack damage by 15%.".to_string(),
        "Passive bonus; always active.".to_string(),
    ];
    rsx! {
        AbilityDescription { description_lines }
    }
}

fn override_key_field_idle() -> Element {
    let label = "A".to_string();
    let is_editing = false;
    let is_special = false;
    let title = "Hotkey".to_string();
    rsx! {
        OverrideKey {
            label,
            is_editing,
            is_special,
            title,
            on_activate: move | _ |
                    {},
        }
    }
}

fn override_key_field_editing() -> Element {
    let label = "A".to_string();
    let is_editing = true;
    let is_special = false;
    let title = "Hotkey".to_string();
    rsx! {
        OverrideKey {
            label,
            is_editing,
            is_special,
            title,
            on_activate: move | _ |
                    {},
        }
    }
}

fn override_key_field_special() -> Element {
    let label = "Esc".to_string();
    let is_editing = false;
    let is_special = true;
    let title = "Hotkey".to_string();
    rsx! {
        OverrideKey {
            label,
            is_editing,
            is_special,
            title,
            on_activate: move | _ |
                    {},
        }
    }
}

fn upgrade_tier_selector_default() -> Element {
    let object_id = WarcraftObjectId::new("AHbz");
    let active_tier_index: usize = 0;
    let total_tier_count: usize = 3;
    let tier_label_text = "Level 1 of 3".to_string();
    let tier_overrides = use_signal(HashMap::<String, usize>::new);
    rsx! {
        UpgradeTier {
            object_id,
            active_tier_index,
            total_tier_count,
            tier_label_text,
            tier_overrides,
        }
    }
}

fn upgrade_section_idle() -> Element {
    let upgrade_hotkey_label = "U".to_string();
    let upgrade_is_editing = false;
    let upgrade_hotkey_is_special = false;
    rsx! {
        UpgradeSection {
            show: true,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_position_click: move |_| {},
            on_hotkey_activate: move |_| {},
        }
    }
}

fn alt_state_section_with_controls() -> Element {
    let alt_name_text: Option<String> = Some("Rooted Form".to_string());
    let alt_description_lines = vec!["The Ancient channels its defensive power.".to_string()];
    let show_alt_controls = true;
    let alt_hotkey_label = "S".to_string();
    let alt_hotkey_is_editing = false;
    let alt_hotkey_is_special_token = false;
    rsx! {
        AltStateSection {
            alt_name_text,
            alt_description_lines,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_position_click: move |_| {},
            on_hotkey_activate: move |_| {},
        }
    }
}

fn alt_state_section_no_controls() -> Element {
    let alt_name_text: Option<String> = Some("Uprooted Form".to_string());
    let alt_description_lines = vec!["The Ancient moves across the battlefield.".to_string()];
    let show_alt_controls = false;
    let alt_hotkey_label = "\u{2013}".to_string();
    let alt_hotkey_is_editing = false;
    let alt_hotkey_is_special_token = false;
    rsx! {
        AltStateSection {
            alt_name_text,
            alt_description_lines,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_position_click: move |_| {},
            on_hotkey_activate: move |_| {},
        }
    }
}

fn alt_position_picker_open() -> Element {
    let object_id = WarcraftObjectId::new("Adef");
    let display_name = "Defend".to_string();
    let off_slot = GridSlotId::ability_off(object_id);
    let picker_slots_vec: Vec<GridSlotId> = vec![off_slot];
    let picker_slots: Rc<[GridSlotId]> = picker_slots_vec.into();
    let alt_position_picker_open = use_signal(|| true);
    rsx! {
        EditorMount {
            ToastMount {
                AltPositionPicker {
                    object_id,
                    display_name,
                    picker_slots,
                    alt_position_picker_open,
                }
            }
        }
    }
}

fn upgrade_position_picker_open() -> Element {
    let upgrade_unit_id = WarcraftObjectId::new("hrtt");
    let display_name = "Siege Engine (upgraded)".to_string();
    let upgrade_slot = GridSlotId::ability(upgrade_unit_id);
    let picker_slots_vec: Vec<GridSlotId> = vec![upgrade_slot];
    let picker_slots: Rc<[GridSlotId]> = picker_slots_vec.into();
    let upgrade_position_picker_open = use_signal(|| true);
    rsx! {
        EditorMount {
            ToastMount {
                UpgradePositionPicker {
                    upgrade_unit_id,
                    display_name,
                    picker_slots,
                    upgrade_position_picker_open,
                }
            }
        }
    }
}

fn tile_override_panel_archmage_blizzard() -> Element {
    let hero_id = fixtures::sample_hero_id();
    let blizzard_id = WarcraftObjectId::new("AHbz");
    let blizzard_slot = GridSlotId::ability(blizzard_id);
    let custom_keys: Option<CustomKeys> = None;
    let from_uprooted = false;
    let from_research = false;
    let upgrade_unit_id: Option<WarcraftObjectId> = None;
    let detail = InspectorDetail::build(
        &blizzard_slot,
        &custom_keys,
        &hero_id,
        from_uprooted,
        from_research,
        upgrade_unit_id,
    );
    let archmage_slots: Rc<[GridSlotId]> = WARCRAFT_DATABASE
        .by_id_and_key(&hero_id)
        .map(|(object_id, _)| {
            WARCRAFT_DATABASE
                .command_card(object_id)
                .filled_slots()
                .collect::<Rc<[GridSlotId]>>()
        })
        .unwrap_or_else(|| Rc::from(Vec::<GridSlotId>::new()));
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    rsx! {
        CustomKeysMount {
            loaded_keys,
            EditorMount {
                TileOverride {
                    detail,
                    active_container_slots: archmage_slots,
                }
            }
        }
    }
}
