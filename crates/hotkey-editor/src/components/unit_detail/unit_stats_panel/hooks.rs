use super::super::derived_stats::DerivedStats;
use super::attributes_column::HeroDisplayData;
use super::combat_column::AttackDisplayData;
use super::leveled_stats::LeveledStats;
use super::props::UnitStatsPanelProps;
use super::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_api::{DefenseType, HeroAttributes, PrimaryAttribute, RegenType};

/// Every display figure the panel renders, resolved from the unit's combat block and
/// the selected hero level.
pub(super) struct UnitStatsPanelModel {
    pub(super) has_hp: bool,
    pub(super) display_hp_text: String,
    pub(super) has_regen: bool,
    pub(super) regen_qualifier: Option<&'static str>,
    pub(super) regen_text: String,
    pub(super) mana_display_text: String,
    pub(super) has_mana: bool,
    pub(super) mana_regen_text: String,
    pub(super) has_mana_regen: bool,
    pub(super) attack: Option<AttackDisplayData>,
    pub(super) has_attack: bool,
    pub(super) armor_text: String,
    pub(super) defense_label: String,
    pub(super) effective_hit_points_text: String,
    pub(super) has_evasion: bool,
    pub(super) evasion_text: String,
    pub(super) defense_icon: Asset,
    pub(super) defense_icon_alt: String,
    pub(super) defense_type: DefenseType,
    pub(super) hero: Option<HeroDisplayData>,
}

/// Resolves all of the panel's stat figures, applying the hero level where relevant.
pub(super) fn use_unit_stats_panel(props: &UnitStatsPanelProps) -> UnitStatsPanelModel {
    let combat = props.combat;
    let hero_attributes = props.hero_attributes;
    let selected_hero_level = props.selected_hero_level;
    let evasion_chance = props.evasion_chance;
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
    let effective_hit_points =
        DerivedStats::effective_hit_points(display_hp, display_armor, evasion_chance);
    let effective_hit_points_text = format!("{effective_hit_points:.0}");
    let has_evasion = evasion_chance > 0.0;
    let evasion_percent = evasion_chance * 100.0;
    let evasion_text = format!("{evasion_percent:.0}%");
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
    let regen_qualifier = match combat.regen_type() {
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
    let attack: Option<AttackDisplayData> = combat.attack().map(|unit_attack| {
        let damage_min = leveled_damage_min.unwrap_or_else(|| unit_attack.damage_min());
        let damage_max = leveled_damage_max.unwrap_or_else(|| unit_attack.damage_max());
        let damage_text = format!("{damage_min}\u{2013}{damage_max}");
        let attack_range = unit_attack.range();
        let cooldown_seconds = unit_attack.cooldown_seconds();
        let speed_text = format!("{cooldown_seconds:.2}s");
        let damage_per_second =
            DerivedStats::damage_per_second(damage_min, damage_max, cooldown_seconds);
        let damage_per_second_text = damage_per_second.map(|value| format!("{value:.1}"));
        let attack_type = unit_attack.attack_type();
        let type_label = attack_type.to_string();
        let type_icon = StatIcon::from(attack_type).asset();
        AttackDisplayData::new(
            damage_text,
            attack_range,
            speed_text,
            damage_per_second_text,
            attack_type,
            type_label,
            type_icon,
        )
    });
    let has_attack = attack.is_some();
    let defense_icon = StatIcon::from(combat.defense_type()).asset();
    let defense_type = combat.defense_type();
    let hero: Option<HeroDisplayData> =
        hero_attributes
            .as_ref()
            .zip(leveled_stats.as_ref())
            .map(|(attributes, stats)| {
                let primary = attributes.primary();
                let primary_icon = StatIcon::from(primary).asset();
                let primary_label = primary.to_string();
                HeroDisplayData::new(
                    primary_icon,
                    primary_label,
                    stats.strength(),
                    attributes.strength_per_level(),
                    stats.agility(),
                    attributes.agility_per_level(),
                    stats.intelligence(),
                    attributes.intelligence_per_level(),
                    primary == PrimaryAttribute::Strength,
                    primary == PrimaryAttribute::Agility,
                    primary == PrimaryAttribute::Intelligence,
                )
            });
    UnitStatsPanelModel {
        has_hp: display_hp > 0,
        display_hp_text,
        has_regen,
        regen_qualifier,
        regen_text,
        mana_display_text,
        has_mana,
        mana_regen_text,
        has_mana_regen,
        attack,
        has_attack,
        armor_text,
        defense_label,
        effective_hit_points_text,
        has_evasion,
        evasion_text,
        defense_icon,
        defense_icon_alt,
        defense_type,
        hero,
    }
}
