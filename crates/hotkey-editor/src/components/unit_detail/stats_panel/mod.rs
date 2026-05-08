mod attack_matchup_cell;
mod attribute_row;
mod attributes_column;
mod combat_column;
mod damage_matchup_row;
mod defense_matchup_cell;
mod defense_matchup_row;
mod leveled_stats;
mod stat_icon;

use dioxus::prelude::*;
use warcraft_api::{HeroAttributes, PrimaryAttribute, RegenType, UnitCombat};

use leveled_stats::LeveledStats;
use stat_icon::StatIcon;

use attributes_column::{AttributesColumn, HeroDisplayData};
use combat_column::{AttackDisplayData, CombatColumn};
use defense_matchup_row::DefenseMatchupRow;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UnitStatsPanelProps {
    pub(crate) combat: UnitCombat,
    pub(crate) hero_attributes: Option<HeroAttributes>,
    pub(crate) selected_hero_level: Signal<u32>,
}

#[component]
pub(crate) fn UnitStatsPanel(props: UnitStatsPanelProps) -> Element {
    let combat = props.combat;
    let hero_attributes = props.hero_attributes;
    let selected_hero_level = props.selected_hero_level;
    let current_level = if hero_attributes.is_some() {
        selected_hero_level()
    } else {
        1
    };
    let leveled_stats = hero_attributes
        .as_ref()
        .map(|attributes| LeveledStats::for_hero(&combat, attributes, current_level));
    let display_hp = leveled_stats
        .as_ref()
        .map(LeveledStats::hit_points)
        .unwrap_or_else(|| combat.hit_points());
    let display_mana = if hero_attributes.is_some() {
        Some(leveled_stats.as_ref().map(LeveledStats::mana).unwrap_or(0))
    } else {
        combat
            .mana_pool()
            .filter(|mana_pool| mana_pool.mana() > 0)
            .map(|mana_pool| mana_pool.mana())
    };
    let display_armor = leveled_stats
        .as_ref()
        .map(LeveledStats::armor)
        .unwrap_or_else(|| combat.armor());
    let leveled_damage_min = leveled_stats.as_ref().map(LeveledStats::damage_min);
    let leveled_damage_max = leveled_stats.as_ref().map(LeveledStats::damage_max);
    let armor_text = format!("{display_armor:.0}");
    let defense_label = combat.defense_type().to_string();
    let defense_icon_alt = format!("{defense_label} defense icon");
    let hp_regen = leveled_stats
        .as_ref()
        .map(LeveledStats::hit_points_regen)
        .unwrap_or_else(|| combat.hit_points_regen());
    let regen_text = format!("+{hp_regen:.2}");
    let regen_qualifier_text = match combat.regen_type() {
        RegenType::Night => Some("at night"),
        RegenType::Blight => Some("on blight"),
        RegenType::Always | RegenType::None => None,
    };
    let has_regen = hp_regen > 0.0 && combat.regen_type() != RegenType::None;
    let mana_display = display_mana.unwrap_or(0);
    let display_hp_text = display_hp.to_string();
    let mana_display_text = mana_display.to_string();
    let has_mana = display_mana
        .map(|mana_value| mana_value > 0)
        .unwrap_or(false);
    let mana_regen = leveled_stats
        .as_ref()
        .map(LeveledStats::mana_regen)
        .unwrap_or_else(|| {
            hero_attributes
                .as_ref()
                .map(HeroAttributes::mana_regen)
                .unwrap_or_else(|| {
                    combat
                        .mana_pool()
                        .map(|mana_pool| mana_pool.mana_regen())
                        .unwrap_or(0.0)
                })
        });
    let has_mana_regen = has_mana && mana_regen > 0.0;
    let mana_regen_text = if has_mana_regen {
        format!("+{mana_regen:.2}")
    } else {
        "+0.00".to_string()
    };
    let mana_value_class = if has_mana {
        "stat-row-value"
    } else {
        "stat-row-value stat-zero"
    };
    let mana_regen_class = if has_mana_regen {
        "stat-row-gain"
    } else {
        "stat-row-gain stat-zero"
    };
    let attack_display: Option<AttackDisplayData> = combat.attack().map(|unit_attack| {
        let damage_min = leveled_damage_min.unwrap_or_else(|| unit_attack.damage_min());
        let damage_max = leveled_damage_max.unwrap_or_else(|| unit_attack.damage_max());
        let attack_range = unit_attack.range();
        let speed_text = format!("{:.2}s", unit_attack.cooldown_seconds());
        let attack_type = unit_attack.attack_type();
        let type_label = attack_type.to_string();
        let type_icon = StatIcon::from(attack_type).asset();
        AttackDisplayData::new(
            damage_min,
            damage_max,
            attack_range,
            speed_text,
            attack_type,
            type_label,
            type_icon,
        )
    });
    let has_attack = attack_display.is_some();
    let defense_icon = StatIcon::from(combat.defense_type()).asset();
    let defense_type = combat.defense_type();
    let hero_display: Option<HeroDisplayData> = hero_attributes
        .as_ref()
        .zip(leveled_stats.as_ref())
        .map(|(attributes, stats)| {
            let primary = attributes.primary();
            let primary_icon = StatIcon::from(primary).asset();
            let primary_label = primary.to_string();
            let strength_value = stats.strength();
            let strength_per_level = attributes.strength_per_level();
            let agility_value = stats.agility();
            let agility_per_level = attributes.agility_per_level();
            let intelligence_value = stats.intelligence();
            let intelligence_per_level = attributes.intelligence_per_level();
            HeroDisplayData {
                primary_is_strength: primary == PrimaryAttribute::Strength,
                primary_is_agility: primary == PrimaryAttribute::Agility,
                primary_is_intelligence: primary == PrimaryAttribute::Intelligence,
                primary_icon,
                primary_label,
                strength_value,
                strength_per_level,
                agility_value,
                agility_per_level,
                intelligence_value,
                intelligence_per_level,
            }
        });
    rsx! {
        div { class: "unit-stats-panel",
            div { class: "stat-column vitality-column",
                if display_hp > 0 {
                    div { class: "stat-row hp",
                        span { class: "stat-row-label", "Hit Points" }
                        span { class: "stat-row-value", {display_hp_text} }
                    }
                    div { class: "stat-row regen-row",
                        span { class: "stat-row-label", "Regeneration" }
                        if has_regen {
                            if let Some(qualifier_text) = regen_qualifier_text {
                                span { class: "regen-qualifier", {qualifier_text} }
                            }
                            span { class: "stat-row-gain", {regen_text} }
                        } else {
                            span { class: "stat-row-gain stat-zero", "+0.00" }
                        }
                    }
                }
                div { class: "stat-row mana",
                    span { class: "stat-row-label", "Mana" }
                    span { class: mana_value_class, {mana_display_text} }
                }
                div { class: "stat-row regen-row mana",
                    span { class: "stat-row-label", "Regeneration" }
                    span { class: mana_regen_class, {mana_regen_text} }
                }
            }
            if let Some(attack_data) = attack_display {
                CombatColumn { attack: attack_data }
            }
            div { class: "stat-column defense-column with-icon",
                div { class: "stat-icon-frame",
                    img {
                        class: "stat-icon",
                        src: defense_icon,
                        alt: defense_icon_alt,
                    }
                }
                div { class: "stat-rows",
                    div { class: "stat-row",
                        span { class: "stat-row-label", "Armor" }
                        span { class: "stat-row-value", {armor_text} }
                    }
                    div { class: "stat-row",
                        span { class: "stat-row-label", "Defense Type" }
                        span { class: "stat-row-value", {defense_label} }
                    }
                    if !has_attack {
                        div { class: "stat-row", "\u{00a0}" }
                        div { class: "stat-row", "\u{00a0}" }
                    }
                    DefenseMatchupRow { defense_type }
                }
            }
            if let Some(hero_data) = hero_display {
                AttributesColumn { hero: hero_data }
            }
        }
    }
}
