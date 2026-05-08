use dioxus::prelude::*;
use num_traits::cast::cast;
use warcraft_api::{
    AttackType, DefenseType, HeroAttributes, PrimaryAttribute, RegenType, UnitCombat,
};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

use super::leveled_stats::LeveledStats;
use super::stat_icon::StatIcon;

const ALL_ATTACK_TYPES: [AttackType; 7] = [
    AttackType::Normal,
    AttackType::Pierce,
    AttackType::Siege,
    AttackType::Magic,
    AttackType::Chaos,
    AttackType::Hero,
    AttackType::Spells,
];

// `Normal` defense exists in the WC3 combat math but no shipping unit uses
// it (verified: zero `DefenseType::Normal` rows in the database). Showing
// a row with always-100% multipliers adds noise without value, so we
// filter it out of both matchup grids.
const DISPLAYED_DEFENSE_TYPES: [DefenseType; 7] = [
    DefenseType::Light,
    DefenseType::Medium,
    DefenseType::Heavy,
    DefenseType::Fortified,
    DefenseType::Hero,
    DefenseType::Divine,
    DefenseType::Unarmored,
];

#[component]
pub(crate) fn UnitStatsPanel(
    combat: UnitCombat,
    hero_attributes: Option<HeroAttributes>,
    selected_hero_level: Signal<u32>,
) -> Element {
    let attack_option = combat.attack();
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
    let has_attack = attack_option.is_some();
    rsx! {
        div { class: "unit-stats-panel",
            div { class: "stat-column vitality-column",
                if display_hp > 0 {
                    {
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
                        rsx! {
                            div { class: "stat-row hp",
                                span { class: "stat-row-label", "Hit Points" }
                                span { class: "stat-row-value", "{display_hp}" }
                            }
                            div { class: "stat-row regen-row",
                                span { class: "stat-row-label", "Regeneration" }
                                if has_regen {
                                    if let Some(qualifier_text) = regen_qualifier_text {
                                        span { class: "regen-qualifier", "{qualifier_text}" }
                                    }
                                    span { class: "stat-row-gain", "{regen_text}" }
                                } else {
                                    span { class: "stat-row-gain stat-zero", "+0.00" }
                                }
                            }
                        }
                    }
                }
                {
                    let mana_display = display_mana.unwrap_or(0);
                    let has_mana = display_mana.map(|mana_value| mana_value > 0).unwrap_or(false);
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
                    rsx! {
                        div { class: "stat-row mana",
                            span { class: "stat-row-label", "Mana" }
                            span { class: mana_value_class, "{mana_display}" }
                        }
                        div { class: "stat-row regen-row mana",
                            span { class: "stat-row-label", "Regeneration" }
                            span { class: mana_regen_class, "{mana_regen_text}" }
                        }
                    }
                }
            }
            if let Some(unit_attack) = attack_option {
                {
                    let damage_min =
                        leveled_damage_min.unwrap_or_else(|| unit_attack.damage_min());
                    let damage_max =
                        leveled_damage_max.unwrap_or_else(|| unit_attack.damage_max());
                    let attack_range = unit_attack.range();
                    let attack_speed_text = format!("{:.2}s", unit_attack.cooldown_seconds());
                    let attack_type_label = unit_attack.attack_type().to_string();
                    let attack_icon = StatIcon::from(unit_attack.attack_type()).asset();
                    rsx! {
                        div { class: "stat-column combat-column with-icon",
                            div { class: "stat-icon-frame",
                                img {
                                    class: "stat-icon",
                                    src: attack_icon,
                                    alt: "{attack_type_label} attack icon",
                                }
                            }
                            div { class: "stat-rows",
                                div { class: "stat-row",
                                    span { class: "stat-row-label", "Damage" }
                                    span { class: "stat-row-value",
                                        "{damage_min}\u{2013}{damage_max}"
                                    }
                                }
                                if attack_range > 0 {
                                    div { class: "stat-row",
                                        span { class: "stat-row-label", "Range" }
                                        span { class: "stat-row-value", "{attack_range}" }
                                    }
                                }
                                div { class: "stat-row",
                                    span { class: "stat-row-label", "Attack Speed" }
                                    span { class: "stat-row-value", "{attack_speed_text}" }
                                }
                                div { class: "stat-row",
                                    span { class: "stat-row-label", "Attack Type" }
                                    span { class: "stat-row-value", "{attack_type_label}" }
                                }
                                DamageMatchupRow { attack_type: unit_attack.attack_type() }
                            }
                        }
                    }
                }
            }
            {
                let defense_icon = StatIcon::from(combat.defense_type()).asset();
                rsx! {
                    div { class: "stat-column defense-column with-icon",
                        div { class: "stat-icon-frame",
                            img {
                                class: "stat-icon",
                                src: defense_icon,
                                alt: "{defense_label} defense icon",
                            }
                        }
                        div { class: "stat-rows",
                            div { class: "stat-row",
                                span { class: "stat-row-label", "Armor" }
                                span { class: "stat-row-value", "{armor_text}" }
                            }
                            div { class: "stat-row",
                                span { class: "stat-row-label", "Defense Type" }
                                span { class: "stat-row-value", "{defense_label}" }
                            }
                            if !has_attack {
                                div { class: "stat-row", "\u{00a0}" }
                                div { class: "stat-row", "\u{00a0}" }
                            }
                            DefenseMatchupRow { defense_type: combat.defense_type() }
                        }
                    }
                }
            }
            if let Some(attributes) = hero_attributes.as_ref()
                && let Some(stats) = leveled_stats.as_ref()
            {
                {
                    let primary = attributes.primary();
                    let primary_icon = StatIcon::from(primary).asset();
                    let primary_label = primary.to_string();
                    rsx! {
                        div { class: "stat-column attributes-column with-icon",
                            div { class: "stat-icon-frame",
                                img {
                                    class: "stat-icon",
                                    src: primary_icon,
                                    alt: "{primary_label} primary attribute icon",
                                }
                            }
                            div { class: "stat-rows",
                                AttributeRow {
                                    label: "Strength",
                                    value: stats.strength(),
                                    per_level: attributes.strength_per_level(),
                                    is_primary: primary == PrimaryAttribute::Strength,
                                }
                                AttributeRow {
                                    label: "Agility",
                                    value: stats.agility(),
                                    per_level: attributes.agility_per_level(),
                                    is_primary: primary == PrimaryAttribute::Agility,
                                }
                                AttributeRow {
                                    label: "Intelligence",
                                    value: stats.intelligence(),
                                    per_level: attributes.intelligence_per_level(),
                                    is_primary: primary == PrimaryAttribute::Intelligence,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DamageMatchupRow(attack_type: AttackType) -> Element {
    let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
    rsx! {
        div { class: "damage-matchup",
            for defense_type in DISPLAYED_DEFENSE_TYPES {
                {
                    let multiplier = effectiveness.against(defense_type);
                    let percent_text = percent_label(multiplier);
                    let cell_class = matchup_cell_class_attacking(multiplier);
                    let defense_label = defense_type.to_string();
                    rsx! {
                        div { class: cell_class, title: "vs {defense_label}",
                            span { class: "matchup-label", "{defense_label}" }
                            span { class: "matchup-value", "{percent_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DefenseMatchupRow(defense_type: DefenseType) -> Element {
    rsx! {
        div { class: "damage-matchup",
            for attack_type in ALL_ATTACK_TYPES {
                {
                    let effectiveness =
                        WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
                    let multiplier = effectiveness.against(defense_type);
                    let percent_text = percent_label(multiplier);
                    let cell_class = matchup_cell_class_defending(multiplier);
                    let attack_label = attack_type.to_string();
                    rsx! {
                        div { class: cell_class, title: "{attack_label} attacks",
                            span { class: "matchup-label", "{attack_label}" }
                            span { class: "matchup-value", "{percent_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeRow(label: &'static str, value: u32, per_level: f32, is_primary: bool) -> Element {
    let row_class = if is_primary {
        "stat-row attribute-row primary"
    } else {
        "stat-row attribute-row"
    };
    let per_level_text = format!("+{per_level:.1}");
    rsx! {
        div { class: row_class,
            span { class: "stat-row-label", "{label}" }
            span { class: "stat-row-value", "{value}" }
            span { class: "stat-row-gain", "{per_level_text}" }
        }
    }
}

fn matchup_cell_class_attacking(multiplier: f32) -> &'static str {
    if multiplier > 1.05 {
        "matchup-cell strong"
    } else if multiplier < 0.95 {
        "matchup-cell weak"
    } else {
        "matchup-cell"
    }
}

fn matchup_cell_class_defending(multiplier: f32) -> &'static str {
    if multiplier > 1.05 {
        "matchup-cell weak"
    } else if multiplier < 0.95 {
        "matchup-cell strong"
    } else {
        "matchup-cell"
    }
}

fn percent_label(multiplier: f32) -> String {
    let percent_int: i32 = cast::<f32, i32>((multiplier * 100.0).round()).unwrap_or(0);
    format!("{percent_int}%")
}
