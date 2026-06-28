use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    AttackDisplayData, AttackMatchupCell, AttributeRow, AttributesColumn, CombatColumn,
    DamageMatchupRow, DefenseMatchupCell, DefenseMatchupRow, DragFollower, DraggingSlot,
    DropTargetCell, HeroDisplayData, HeroLevelOption, StatIcon, UnitCommandGrids, UnitDetailHeader,
    UnitDetailPanel, UnitStatsPanel,
};
use warcraft_api::{
    AttackType, DefenseType, PrimaryAttribute, Race, UnitCombat, WarcraftObjectMeta,
};
use warcraft_database::{ObjectLookup, WARCRAFT_DATABASE};
use warcraft_keybinds::{GridSlotId, UnitCommandSlots};

use super::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "Unit detail",
            "AttributeRow",
            "Default",
            attribute_row_default,
        ),
        Story::new(
            "Unit detail",
            "AttributeRow",
            "Primary",
            attribute_row_primary,
        ),
        Story::single(
            "Unit detail",
            "AttackMatchupCell",
            attack_matchup_cell_normal_vs_heavy,
        ),
        Story::single(
            "Unit detail",
            "DefenseMatchupCell",
            defense_matchup_cell_heavy_vs_normal,
        ),
        Story::single("Unit detail", "DamageMatchupRow", damage_matchup_row_normal),
        Story::single(
            "Unit detail",
            "DefenseMatchupRow",
            defense_matchup_row_heavy,
        ),
        Story::new(
            "Unit detail",
            "HeroLevelOption",
            "Selected",
            hero_level_option_selected,
        ),
        Story::new(
            "Unit detail",
            "HeroLevelOption",
            "Unselected",
            hero_level_option_unselected,
        ),
        Story::new(
            "Unit detail",
            "UnitDetailHeader",
            "No portrait",
            unit_detail_header_no_portrait,
        ),
        Story::new(
            "Unit detail",
            "UnitDetailHeader",
            "Hero level picker",
            unit_detail_header_hero,
        ),
        Story::new(
            "Unit detail",
            "UnitStatsPanel",
            "Empty unit",
            unit_stats_panel_empty,
        ),
        Story::new(
            "Unit detail",
            "UnitStatsPanel",
            "Archmage (real hero data)",
            unit_stats_panel_archmage,
        ),
        Story::new(
            "Unit detail",
            "UnitStatsPanel",
            "Footman (real unit data)",
            unit_stats_panel_footman,
        ),
        Story::single(
            "Unit detail",
            "AttributesColumn",
            attributes_column_archmage,
        ),
        Story::single("Unit detail", "CombatColumn", combat_column_footman),
        Story::single(
            "Unit detail",
            "UnitCommandGrids",
            unit_command_grids_footman,
        ),
        Story::new(
            "Unit detail",
            "UnitDetailPanel",
            "Footman",
            unit_detail_panel_footman,
        ),
        Story::new(
            "Unit detail",
            "UnitDetailPanel",
            "Archmage",
            unit_detail_panel_archmage,
        ),
    ]
}

fn attribute_row_default() -> Element {
    let label = "Hit Points";
    let value: u32 = 500;
    let per_level: f32 = 0.0;
    let is_primary = false;
    rsx! {
        AttributeRow { label, value, per_level, is_primary }
    }
}

fn attribute_row_primary() -> Element {
    let label = "Strength";
    let value: u32 = 25;
    let per_level: f32 = 2.5;
    let is_primary = true;
    rsx! {
        AttributeRow { label, value, per_level, is_primary }
    }
}

fn attack_matchup_cell_normal_vs_heavy() -> Element {
    let attack_type = AttackType::Normal;
    let defense_type = DefenseType::Heavy;
    rsx! {
        AttackMatchupCell { defense_type, attack_type }
    }
}

fn defense_matchup_cell_heavy_vs_normal() -> Element {
    let attack_type = AttackType::Normal;
    let defense_type = DefenseType::Heavy;
    rsx! {
        DefenseMatchupCell { attack_type, defense_type }
    }
}

fn damage_matchup_row_normal() -> Element {
    let attack_type = AttackType::Normal;
    rsx! {
        DamageMatchupRow { attack_type }
    }
}

fn defense_matchup_row_heavy() -> Element {
    let defense_type = DefenseType::Heavy;
    rsx! {
        DefenseMatchupRow { defense_type }
    }
}

fn hero_level_option_selected() -> Element {
    let level_index: u32 = 5;
    let current_level: u32 = 5;
    let initial_level: u32 = 5;
    let initial_open: bool = false;
    let selected_hero_level = use_signal(|| initial_level);
    let level_picker_open = use_signal(|| initial_open);
    rsx! {
        HeroLevelOption {
            level_index,
            current_level,
            selected_hero_level,
            level_picker_open,
        }
    }
}

fn hero_level_option_unselected() -> Element {
    let level_index: u32 = 3;
    let current_level: u32 = 5;
    let initial_level: u32 = 5;
    let initial_open: bool = false;
    let selected_hero_level = use_signal(|| initial_level);
    let level_picker_open = use_signal(|| initial_open);
    rsx! {
        HeroLevelOption {
            level_index,
            current_level,
            selected_hero_level,
            level_picker_open,
        }
    }
}

fn unit_detail_header_no_portrait() -> Element {
    let unit_name = "Footman";
    let unit_id = "hfoo".to_string();
    let portrait_url = None;
    let has_hero_attributes = false;
    let initial_level: u32 = 1;
    let initial_open: bool = false;
    let selected_hero_level = use_signal(|| initial_level);
    let level_picker_open = use_signal(|| initial_open);
    rsx! {
        UnitDetailHeader {
            unit_name,
            unit_id,
            portrait_url,
            has_hero_attributes,
            selected_hero_level,
            level_picker_open,
        }
    }
}

fn unit_detail_header_hero() -> Element {
    let unit_name = "Archmage";
    let unit_id = "Hamg".to_string();
    let portrait_url = None;
    let has_hero_attributes = true;
    let initial_level: u32 = 3;
    let initial_open: bool = false;
    let selected_hero_level = use_signal(|| initial_level);
    let level_picker_open = use_signal(|| initial_open);
    rsx! {
        UnitDetailHeader {
            unit_name,
            unit_id,
            portrait_url,
            has_hero_attributes,
            selected_hero_level,
            level_picker_open,
        }
    }
}

fn unit_stats_panel_empty() -> Element {
    let combat = UnitCombat::EMPTY;
    let hero_attributes = None;
    let initial_level: u32 = 1;
    let selected_hero_level = use_signal(|| initial_level);
    let evasion_chance: f32 = 0.0;
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion_chance,
        }
    }
}

fn unit_stats_panel_archmage() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_hero_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Archmage not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let combat = *unit_meta.combat();
    let hero_attributes = unit_meta.hero_attributes().copied();
    let initial_level: u32 = 1;
    let selected_hero_level = use_signal(|| initial_level);
    let evasion_chance: f32 = 0.0;
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion_chance,
        }
    }
}

fn unit_stats_panel_footman() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_unit_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Footman not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let combat = *unit_meta.combat();
    let hero_attributes = None;
    let initial_level: u32 = 1;
    let selected_hero_level = use_signal(|| initial_level);
    let evasion_chance: f32 = 0.0;
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion_chance,
        }
    }
}

fn attributes_column_archmage() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_hero_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Archmage not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let Some(attributes) = unit_meta.hero_attributes() else {
        return rsx! { "Unit has no hero attributes." };
    };
    let primary = attributes.primary();
    let primary_icon = StatIcon::from(primary).asset();
    let primary_label = primary.to_string();
    let strength_value = attributes.strength();
    let strength_per_level = attributes.strength_per_level();
    let agility_value = attributes.agility();
    let agility_per_level = attributes.agility_per_level();
    let intelligence_value = attributes.intelligence();
    let intelligence_per_level = attributes.intelligence_per_level();
    let primary_is_strength = primary == PrimaryAttribute::Strength;
    let primary_is_agility = primary == PrimaryAttribute::Agility;
    let primary_is_intelligence = primary == PrimaryAttribute::Intelligence;
    let hero = HeroDisplayData::new(
        primary_icon,
        primary_label,
        strength_value,
        strength_per_level,
        agility_value,
        agility_per_level,
        intelligence_value,
        intelligence_per_level,
        primary_is_strength,
        primary_is_agility,
        primary_is_intelligence,
    );
    rsx! {
        AttributesColumn { hero }
    }
}

fn combat_column_footman() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_unit_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Footman not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let combat = unit_meta.combat();
    let Some(unit_attack) = combat.attack() else {
        return rsx! { "Unit has no attack." };
    };
    let damage_min = unit_attack.damage_min();
    let damage_max = unit_attack.damage_max();
    let damage_text = format!("{damage_min}\u{2013}{damage_max}");
    let attack_range = unit_attack.range();
    let cooldown_seconds = unit_attack.cooldown_seconds();
    let speed_text = format!("{cooldown_seconds:.2}s");
    let attack_type = unit_attack.attack_type();
    let type_label = attack_type.to_string();
    let type_icon = StatIcon::from(attack_type).asset();
    let attack = AttackDisplayData::new(
        damage_text,
        attack_range,
        speed_text,
        None,
        attack_type,
        type_label,
        type_icon,
    );
    rsx! {
        CombatColumn { attack }
    }
}

fn unit_command_grids_footman() -> Element {
    let unit_id = fixtures::sample_unit_id();
    let lookup_result = WARCRAFT_DATABASE.by_id_and_key(&unit_id);
    let command_card_slots: Rc<[GridSlotId]> = lookup_result
        .map(|(obj_id, _)| {
            WARCRAFT_DATABASE
                .command_card(obj_id)
                .filled_slots()
                .collect::<Rc<[GridSlotId]>>()
        })
        .unwrap_or_else(|| Rc::from(Vec::<GridSlotId>::new()));
    let loaded_keys = use_signal(|| None);
    let selected_slot = use_signal(|| None);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    rsx! {
        UnitCommandGrids {
            unit_id,
            race: Race::Human,
            command_card_slots,
            build_menu_slots: None,
            uprooted_menu_slots: None,
            research_menu_slots: None,
            loaded_keys,
            selected_slot,
            selected_from_research,
            selected_from_uprooted,
            tier_overrides,
            dragging_slot,
            drop_target_cell,
            drag_follower,
            grid_layout,
            update_hotkeys_on_move,
            hotkey_assign_request,
        }
    }
}

fn unit_detail_panel_footman() -> Element {
    let selected_unit_id = use_signal(|| Some(fixtures::sample_unit_id()));
    let selected_slot = use_signal(|| None);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let loaded_keys = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let active_race = use_signal(|| Race::Human);
    rsx! {
        UnitDetailPanel {
            active_race,
            selected_unit_id,
            selected_slot,
            selected_from_research,
            selected_from_uprooted,
            tier_overrides,
            dragging_slot,
            drop_target_cell,
            drag_follower,
            loaded_keys,
            grid_layout,
            update_hotkeys_on_move,
            hotkey_assign_request,
        }
    }
}

fn unit_detail_panel_archmage() -> Element {
    let selected_unit_id = use_signal(|| Some(fixtures::sample_hero_id()));
    let selected_slot = use_signal(|| None);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let loaded_keys = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let active_race = use_signal(|| Race::Human);
    rsx! {
        UnitDetailPanel {
            active_race,
            selected_unit_id,
            selected_slot,
            selected_from_research,
            selected_from_uprooted,
            tier_overrides,
            dragging_slot,
            drop_target_cell,
            drag_follower,
            loaded_keys,
            grid_layout,
            update_hotkeys_on_move,
            hotkey_assign_request,
        }
    }
}
