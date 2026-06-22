use num_traits::cast::cast;
use warcraft_api::{UnitMeta, WarcraftObjectMeta};
use warcraft_database::ObjectLookup;

/// Derived combat figures shown next to the raw stats: damage-per-second and
/// effective hit points. The arithmetic is pure (so it unit-tests without a
/// browser); the evasion input is resolved separately from the unit's
/// abilities, since evasion is granted by abilities, not a base stat.
pub(crate) struct DerivedStats;

impl DerivedStats {
    /// Mean damage per second over the attack cooldown. `None` when there is
    /// no real attack (a non-positive cooldown).
    pub(crate) fn damage_per_second(
        damage_min: u32,
        damage_max: u32,
        cooldown_seconds: f32,
    ) -> Option<f32> {
        if cooldown_seconds <= 0.0 {
            return None;
        }
        let damage_min_float: f32 = cast(damage_min).unwrap_or(0.0);
        let damage_max_float: f32 = cast(damage_max).unwrap_or(0.0);
        let average_damage = (damage_min_float + damage_max_float) / 2.0;
        Some(average_damage / cooldown_seconds)
    }

    /// Effective hit points: how much raw incoming damage the unit survives,
    /// scaling its hit points by armor (each point ≈ +6% EHP) and evasion
    /// (survivability ×1/(1−evasion)). Mirrors Warcraft III's armor model.
    pub(crate) fn effective_hit_points(hit_points: u32, armor: f32, evasion_chance: f32) -> f32 {
        let hit_points_float: f32 = cast(hit_points).unwrap_or(0.0);
        let armor_multiplier = Self::armor_multiplier(armor);
        let evasion_multiplier = Self::evasion_multiplier(evasion_chance);
        hit_points_float * armor_multiplier * evasion_multiplier
    }

    /// EHP factor from armor. Positive armor adds 6% per point; negative armor
    /// amplifies incoming damage by `2 − 0.94^(−armor)`.
    fn armor_multiplier(armor: f32) -> f32 {
        let armor_step: f32 = 0.06;
        let armor_falloff: f32 = 0.94;
        if armor >= 0.0 {
            1.0 + armor_step * armor
        } else {
            let damage_amplification = 2.0 - armor_falloff.powf(-armor);
            1.0 / damage_amplification
        }
    }

    /// EHP factor from evasion. Clamped below 1.0 so the figure stays finite;
    /// no Warcraft III unit reaches anywhere near full evasion.
    fn evasion_multiplier(evasion_chance: f32) -> f32 {
        let evasion_ceiling: f32 = 0.95;
        let clamped_evasion = evasion_chance.clamp(0.0, evasion_ceiling);
        1.0 / (1.0 - clamped_evasion)
    }

    /// Highest evasion chance the unit can field, across its abilities and
    /// hero abilities, at the ability's full level. Evasion abilities do not
    /// stack — the strongest wins — so this is the per-unit dodge chance.
    /// `0.0` when the unit has no evasion source.
    pub(crate) fn unit_evasion_chance(unit_meta: &UnitMeta) -> f32 {
        let standard_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();
        let ability_lists = [standard_abilities, hero_abilities];
        let mut best_evasion: f32 = 0.0;
        for ability_list in ability_lists {
            for ability_id in ability_list {
                let ability_id_value = ability_id.value();
                let Some(ability_object) = ObjectLookup::by_id(ability_id_value) else {
                    continue;
                };
                let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
                    continue;
                };
                let evasion_chances = ability_meta.evasion_chances();
                for chance in evasion_chances {
                    if chance > best_evasion {
                        best_evasion = chance;
                    }
                }
            }
        }
        best_evasion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dps_is_mean_damage_over_cooldown() {
        // Ogre Mauler: 24–27 damage at 1.35s cooldown → 25.5 / 1.35 ≈ 18.89.
        let damage_per_second = DerivedStats::damage_per_second(24, 27, 1.35).unwrap();
        assert!((damage_per_second - 18.888).abs() < 0.01);
    }

    #[test]
    fn dps_is_none_without_a_real_attack() {
        assert!(DerivedStats::damage_per_second(0, 0, 0.0).is_none());
    }

    #[test]
    fn ehp_adds_six_percent_per_armor_point() {
        // 850 HP, 3 armor, no evasion → 850 × (1 + 0.18) = 1003.
        let effective_hit_points = DerivedStats::effective_hit_points(850, 3.0, 0.0);
        assert!((effective_hit_points - 1003.0).abs() < 0.5);
    }

    #[test]
    fn ehp_folds_in_evasion() {
        // 1000 HP, 0 armor, 20% evasion → 1000 / 0.8 = 1250.
        let effective_hit_points = DerivedStats::effective_hit_points(1000, 0.0, 0.2);
        assert!((effective_hit_points - 1250.0).abs() < 0.5);
    }
}
