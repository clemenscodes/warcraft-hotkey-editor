use super::fixtures;
use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::UnitDetail;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGrids;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_header::UnitDetailHeader;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_header::components::unit_detail_title::components::unit_name_row::components::hero_level_picker::components::hero_level_menu::components::hero_level_option::HeroLevelOption;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::UnitStatsPanel;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::combat_column::components::combat_rows::components::damage_matchup_row::components::attack_matchup::AttackMatchup;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::attributes_column::AttributesColumn;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::combat_column::CombatColumn;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::combat_column::components::combat_rows::components::damage_matchup_row::DamageMatchupRow;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::defense_column::components::defense_rows::components::defense_matchup_row::components::defense_matchup::DefenseMatchup;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::defense_column::components::defense_rows::components::defense_matchup_row::DefenseMatchupRow;
use super::editor_mount::EditorMount;
use std::rc::Rc;

use warcraft_api::{AttackType, DefenseType, Race, UnitCombat, WarcraftObjectMeta};

use warcraft_api::{ObjectLookup, WARCRAFT_DATABASE};
use warcraft_keybinds::{
    AttackRange, AttackSpeed, AttackStatistics, AttributeStatistic, DamageRange, Evasion,
    GridSlotId, HeroStatistics, UnitCommandSlots,
};

pub fn stories() -> Vec<Story> {
    vec![
        Story::single(
            "Unit detail",
            "AttackMatchup",
            attack_matchup_normal_vs_heavy,
        ),
        Story::single(
            "Unit detail",
            "DefenseMatchup",
            defense_matchup_heavy_vs_normal,
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
            "UnitDetail",
            "Footman",
            unit_detail_panel_footman,
        ),
        Story::new(
            "Unit detail",
            "UnitDetail",
            "Archmage",
            unit_detail_panel_archmage,
        ),
    ]
}

fn attack_matchup_normal_vs_heavy() -> Element {
    let attack_type = AttackType::Normal;
    let defense_type = DefenseType::Heavy;
    rsx! {
        AttackMatchup { defense_type, attack_type }
    }
}

fn defense_matchup_heavy_vs_normal() -> Element {
    let attack_type = AttackType::Normal;
    let defense_type = DefenseType::Heavy;
    rsx! {
        DefenseMatchup { attack_type, defense_type }
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
    let unit_id = ObjectLookup::resolve_raw("hfoo").expect("known object id");
    let portrait_url = None;
    let has_hero_attributes = false;
    rsx! {
        EditorMount {
            UnitDetailHeader {
                unit_name,
                unit_id,
                portrait_url,
                has_hero_attributes,
            }
        }
    }
}

fn unit_detail_header_hero() -> Element {
    let unit_name = "Archmage";
    let unit_id = ObjectLookup::resolve_raw("Hamg").expect("known object id");
    let portrait_url = None;
    let has_hero_attributes = true;
    rsx! {
        EditorMount {
            UnitDetailHeader {
                unit_name,
                unit_id,
                portrait_url,
                has_hero_attributes,
            }
        }
    }
}

fn unit_stats_panel_empty() -> Element {
    let combat = UnitCombat::EMPTY;
    let hero_attributes = None;
    let initial_level: u32 = 1;
    let selected_hero_level = use_signal(|| initial_level);
    let evasion = Evasion::default();
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion,
        }
    }
}

fn unit_stats_panel_archmage() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_hero_id());
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
    let evasion = Evasion::default();
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion,
        }
    }
}

fn unit_stats_panel_footman() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_unit_id());
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
    let evasion = Evasion::default();
    rsx! {
        UnitStatsPanel {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion,
        }
    }
}

fn attributes_column_archmage() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_hero_id());
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
    let strength_value = attributes.strength();
    let strength_per_level = attributes.strength_per_level();
    let agility_value = attributes.agility();
    let agility_per_level = attributes.agility_per_level();
    let intelligence_value = attributes.intelligence();
    let intelligence_per_level = attributes.intelligence_per_level();
    let strength = AttributeStatistic::new(strength_value, strength_per_level);
    let agility = AttributeStatistic::new(agility_value, agility_per_level);
    let intelligence = AttributeStatistic::new(intelligence_value, intelligence_per_level);
    let hero_statistics = HeroStatistics::new(strength, agility, intelligence, primary);
    let hero = Some(hero_statistics);
    rsx! {
        AttributesColumn { hero }
    }
}

fn combat_column_footman() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_unit_id());
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
    let damage_minimum = unit_attack.damage_min();
    let damage_maximum = unit_attack.damage_max();
    let damage = DamageRange::new(damage_minimum, damage_maximum);
    let attack_range_value = unit_attack.range();
    let range = AttackRange::new(attack_range_value);
    let cooldown_seconds = unit_attack.cooldown_seconds();
    let speed = AttackSpeed::new(cooldown_seconds);
    let damage_per_second = None;
    let attack_type = unit_attack.attack_type();
    let attack_statistics =
        AttackStatistics::new(damage, range, speed, damage_per_second, attack_type);
    let attack = Some(attack_statistics);
    rsx! {
        CombatColumn { attack }
    }
}

fn unit_command_grids_footman() -> Element {
    let unit_id = fixtures::sample_unit_id();
    let command_card_slots: Rc<[GridSlotId]> = WARCRAFT_DATABASE
        .command_card(unit_id)
        .filled_slots()
        .collect::<Rc<[GridSlotId]>>();
    rsx! {
        EditorMount {
            UnitCommandGrids {
                unit_id,
                race: Race::Human,
                command_card_slots,
                build_menu_slots: None,
                uprooted_menu_slots: None,
                research_menu_slots: None,
            }
        }
    }
}

fn unit_detail_panel_footman() -> Element {
    let selected_unit_id = Some(fixtures::sample_unit_id());
    rsx! {
        EditorMount {
            active_race: Race::Human,
            selected_unit_id,
            UnitDetail {}
        }
    }
}

fn unit_detail_panel_archmage() -> Element {
    let selected_unit_id = Some(fixtures::sample_hero_id());
    rsx! {
        EditorMount {
            active_race: Race::Human,
            selected_unit_id,
            UnitDetail {}
        }
    }
}
