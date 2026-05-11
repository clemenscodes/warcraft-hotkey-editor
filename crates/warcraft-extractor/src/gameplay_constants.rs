use std::path::PathBuf;

use warcraft_api::{
    AgilityBonuses, DamageEffectiveness, DamageMatrix, GameplayConstants, IntelligenceBonuses,
    StrengthBonuses,
};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub static GAMEPLAY_CONSTANTS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: GameplayConstantsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: GameplayConstantsExtraction::process,
};

struct GameplayConstantsExtraction;

impl GameplayConstantsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        let lower_filename = filename.to_ascii_lowercase();
        lower_filename == "miscgame.txt" && path.contains("war3.w3mod:units")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let constants = parse_miscgame(text);
        let boxed_constants = Box::new(constants);
        Ok(ExtractResult::GameplayConstants(boxed_constants))
    }
}

fn parse_damage_bonus(value: &str) -> DamageEffectiveness {
    let mut multipliers: [f32; 8] = [1.0; 8];
    for (parsed_index, raw_part) in value.split(',').enumerate() {
        if parsed_index >= multipliers.len() {
            break;
        }
        let trimmed_part = raw_part.trim();
        if let Ok(parsed_value) = trimmed_part.parse::<f32>() {
            multipliers[parsed_index] = parsed_value;
        }
    }
    DamageEffectiveness::new(multipliers)
}

fn parse_miscgame(text: &str) -> GameplayConstants {
    let mut str_attack_bonus: f32 = 1.0;
    let mut str_hit_point_bonus: u32 = 25;
    let mut str_regen_bonus: f32 = 0.05;
    let mut int_mana_bonus: u32 = 15;
    let mut int_regen_bonus: f32 = 0.05;
    let mut agi_defense_bonus: f32 = 0.30;
    let mut agi_attack_speed_bonus: f32 = 0.02;
    let mut max_hero_level: u32 = 10;
    let defaults = GameplayConstants::default();
    let mut damage_normal = defaults.damage_effectiveness(warcraft_api::AttackType::Normal);
    let mut damage_pierce = defaults.damage_effectiveness(warcraft_api::AttackType::Pierce);
    let mut damage_siege = defaults.damage_effectiveness(warcraft_api::AttackType::Siege);
    let mut damage_magic = defaults.damage_effectiveness(warcraft_api::AttackType::Magic);
    let mut damage_chaos = defaults.damage_effectiveness(warcraft_api::AttackType::Chaos);
    let mut damage_spells = defaults.damage_effectiveness(warcraft_api::AttackType::Spells);
    let mut damage_hero = defaults.damage_effectiveness(warcraft_api::AttackType::Hero);

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        let comment_split = trimmed.split("//").next().unwrap_or(trimmed);
        let Some((key, value)) = comment_split.split_once('=') else {
            continue;
        };
        let key_trimmed = key.trim();
        let value_trimmed = value.trim();
        match key_trimmed {
            "StrAttackBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<f32>() {
                    str_attack_bonus = parsed;
                }
            }
            "StrHitPointBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<u32>() {
                    str_hit_point_bonus = parsed;
                }
            }
            "StrRegenBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<f32>() {
                    str_regen_bonus = parsed;
                }
            }
            "IntManaBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<u32>() {
                    int_mana_bonus = parsed;
                }
            }
            "IntRegenBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<f32>() {
                    int_regen_bonus = parsed;
                }
            }
            "AgiDefenseBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<f32>() {
                    agi_defense_bonus = parsed;
                }
            }
            "AgiAttackSpeedBonus" => {
                if let Ok(parsed) = value_trimmed.parse::<f32>() {
                    agi_attack_speed_bonus = parsed;
                }
            }
            "MaxHeroLevel" => {
                if let Ok(parsed) = value_trimmed.parse::<u32>() {
                    max_hero_level = parsed;
                }
            }
            "DamageBonusNormal" => damage_normal = parse_damage_bonus(value_trimmed),
            "DamageBonusPierce" => damage_pierce = parse_damage_bonus(value_trimmed),
            "DamageBonusSiege" => damage_siege = parse_damage_bonus(value_trimmed),
            "DamageBonusMagic" => damage_magic = parse_damage_bonus(value_trimmed),
            "DamageBonusChaos" => damage_chaos = parse_damage_bonus(value_trimmed),
            "DamageBonusSpells" => damage_spells = parse_damage_bonus(value_trimmed),
            "DamageBonusHero" => damage_hero = parse_damage_bonus(value_trimmed),
            _ => {}
        }
    }

    let strength_bonuses =
        StrengthBonuses::new(str_attack_bonus, str_hit_point_bonus, str_regen_bonus);
    let intelligence_bonuses = IntelligenceBonuses::new(int_mana_bonus, int_regen_bonus);
    let agility_bonuses = AgilityBonuses::new(agi_defense_bonus, agi_attack_speed_bonus);
    let damage_matrix = DamageMatrix::new(
        damage_normal,
        damage_pierce,
        damage_siege,
        damage_magic,
        damage_chaos,
        damage_spells,
        damage_hero,
    );
    GameplayConstants::new(
        strength_bonuses,
        intelligence_bonuses,
        agility_bonuses,
        max_hero_level,
        damage_matrix,
    )
}
