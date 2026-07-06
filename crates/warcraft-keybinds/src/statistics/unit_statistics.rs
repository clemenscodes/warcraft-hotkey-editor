//! The aggregate [`UnitStatistics`] and the domain arithmetic that produces it: the
//! hero-level attribute scaling and the derived figures (effective hit points,
//! damage per second). This is combat math with no browser dependency, so it lives
//! in the pure crate and unit-tests natively; the renderer only reads the result.

use super::attack::AttackStatistics;
use super::hero::{AttributeStatistic, HeroStatistics};
use super::values::{
    Armor, AttackRange, AttackSpeed, DamagePerSecond, DamageRange, EffectiveHitPoints, Evasion,
    HitPoints, HitPointsRegen, Mana, ManaRegen,
};
use warcraft_api::{DefenseType, HeroAttributes, PrimaryAttribute, UnitAttack, UnitCombat};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

/// Widens a count into `f32` for arithmetic. The lossy `as` cast is confined to this
/// `From` body (RUST_STYLE permits `as` only inside `From`/`TryFrom` impls); stat
/// counts are small, so the conversion is exact.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct AsFloat {
    value: f32,
}

impl From<u32> for AsFloat {
    fn from(value: u32) -> Self {
        Self {
            value: value as f32,
        }
    }
}

/// Floors a non-negative float back into a count, clamping below at zero. The `as`
/// cast is confined to this `From` body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct FlooredCount {
    value: u32,
}

impl From<f32> for FlooredCount {
    fn from(value: f32) -> Self {
        let floored = value.floor().max(0.0);
        Self {
            value: floored as u32,
        }
    }
}

/// A hero's figures scaled to the selected level: the three attributes and every
/// stat the primary attribute raises. Private to this module — the public surface
/// is [`UnitStatistics`], built from these.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct LeveledFigures {
    strength: u32,
    agility: u32,
    intelligence: u32,
    hit_points: u32,
    hit_points_regen: f32,
    mana: u32,
    mana_regen: f32,
    armor: f32,
    damage_minimum: u32,
    damage_maximum: u32,
}

impl LeveledFigures {
    fn for_hero(combat: &UnitCombat, attributes: &HeroAttributes, level: u32) -> Self {
        let levels_added = level.saturating_sub(1);
        let levels_added_float = AsFloat::from(levels_added).value;
        let base_strength = attributes.strength();
        let base_agility = attributes.agility();
        let base_intelligence = attributes.intelligence();
        let base_strength_float = AsFloat::from(base_strength).value;
        let base_agility_float = AsFloat::from(base_agility).value;
        let base_intelligence_float = AsFloat::from(base_intelligence).value;
        let strength_total =
            base_strength_float + attributes.strength_per_level() * levels_added_float;
        let agility_total =
            base_agility_float + attributes.agility_per_level() * levels_added_float;
        let intelligence_total =
            base_intelligence_float + attributes.intelligence_per_level() * levels_added_float;
        let strength = FlooredCount::from(strength_total).value.max(base_strength);
        let agility = FlooredCount::from(agility_total).value.max(base_agility);
        let intelligence = FlooredCount::from(intelligence_total)
            .value
            .max(base_intelligence);
        let strength_delta = strength.saturating_sub(base_strength);
        let agility_delta = agility.saturating_sub(base_agility);
        let intelligence_delta = intelligence.saturating_sub(base_intelligence);
        let strength_float = AsFloat::from(strength).value;
        let agility_delta_float = AsFloat::from(agility_delta).value;
        let intelligence_float = AsFloat::from(intelligence).value;
        let hit_points = combat.hit_points()
            + strength_delta * WARCRAFT_GAMEPLAY_CONSTANTS.str_hit_point_bonus();
        let hit_points_regen = combat.hit_points_regen()
            + strength_float * WARCRAFT_GAMEPLAY_CONSTANTS.str_regen_bonus();
        let mana =
            attributes.mana() + intelligence_delta * WARCRAFT_GAMEPLAY_CONSTANTS.int_mana_bonus();
        let mana_regen = attributes.mana_regen()
            + intelligence_float * WARCRAFT_GAMEPLAY_CONSTANTS.int_regen_bonus();
        let armor =
            combat.armor() + agility_delta_float * WARCRAFT_GAMEPLAY_CONSTANTS.agi_defense_bonus();
        let primary_now = match attributes.primary() {
            PrimaryAttribute::Strength => strength,
            PrimaryAttribute::Agility => agility,
            PrimaryAttribute::Intelligence => intelligence,
        };
        let primary_base = match attributes.primary() {
            PrimaryAttribute::Strength => base_strength,
            PrimaryAttribute::Agility => base_agility,
            PrimaryAttribute::Intelligence => base_intelligence,
        };
        let primary_delta = primary_now.saturating_sub(primary_base);
        let primary_delta_float = AsFloat::from(primary_delta).value;
        let attack_bonus_float =
            primary_delta_float * WARCRAFT_GAMEPLAY_CONSTANTS.str_attack_bonus();
        let primary_delta_attack = FlooredCount::from(attack_bonus_float)
            .value
            .max(primary_delta);
        let attack_minimum_base = combat.attack().map(UnitAttack::damage_min).unwrap_or(0);
        let attack_maximum_base = combat.attack().map(UnitAttack::damage_max).unwrap_or(0);
        Self {
            strength,
            agility,
            intelligence,
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
            armor,
            damage_minimum: attack_minimum_base + primary_delta_attack,
            damage_maximum: attack_maximum_base + primary_delta_attack,
        }
    }
}

/// A unit's complete combat statistics as strongly-typed domain values, resolved for
/// the selected hero level. This is the single value the renderer's statistics card
/// receives; it renders each typed figure and never re-derives a number.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct UnitStatistics {
    hit_points: HitPoints,
    hit_points_regen: HitPointsRegen,
    mana: Mana,
    mana_regen: ManaRegen,
    armor: Armor,
    defense_type: DefenseType,
    effective_hit_points: EffectiveHitPoints,
    evasion: Evasion,
    attack: Option<AttackStatistics>,
    hero: Option<HeroStatistics>,
}

impl UnitStatistics {
    /// Resolve every figure from the unit's combat block, its optional hero
    /// attributes at `hero_level`, and the evasion chance granted by its abilities.
    pub fn compute(
        combat: &UnitCombat,
        hero_attributes: Option<&HeroAttributes>,
        hero_level: u32,
        evasion_chance: f32,
    ) -> Self {
        let leveled = hero_attributes
            .map(|attributes| LeveledFigures::for_hero(combat, attributes, hero_level));
        let hit_points_amount = leveled
            .as_ref()
            .map(|figures| figures.hit_points)
            .unwrap_or_else(|| combat.hit_points());
        let hit_points_regen_rate = leveled
            .as_ref()
            .map(|figures| figures.hit_points_regen)
            .unwrap_or_else(|| combat.hit_points_regen());
        let regen_type = combat.regen_type();
        let armor_amount = leveled
            .as_ref()
            .map(|figures| figures.armor)
            .unwrap_or_else(|| combat.armor());
        let mana_amount = leveled
            .as_ref()
            .map(|figures| figures.mana)
            .unwrap_or_else(|| combat.mana_pool().map(|pool| pool.mana()).unwrap_or(0));
        let mana_regen_rate = leveled
            .as_ref()
            .map(|figures| figures.mana_regen)
            .unwrap_or_else(|| {
                combat
                    .mana_pool()
                    .map(|pool| pool.mana_regen())
                    .unwrap_or(0.0)
            });
        let defense_type = combat.defense_type();
        let effective_hit_points_amount =
            Self::effective_hit_points_from(hit_points_amount, armor_amount, evasion_chance);
        let attack = combat.attack().map(|unit_attack| {
            let damage_minimum = leveled
                .as_ref()
                .map(|figures| figures.damage_minimum)
                .unwrap_or_else(|| unit_attack.damage_min());
            let damage_maximum = leveled
                .as_ref()
                .map(|figures| figures.damage_maximum)
                .unwrap_or_else(|| unit_attack.damage_max());
            let damage = DamageRange::new(damage_minimum, damage_maximum);
            let range = AttackRange::new(unit_attack.range());
            let cooldown_seconds = unit_attack.cooldown_seconds();
            let speed = AttackSpeed::new(cooldown_seconds);
            let damage_per_second_amount = Self::damage_per_second(damage, cooldown_seconds);
            let damage_per_second = damage_per_second_amount.map(DamagePerSecond::new);
            let attack_type = unit_attack.attack_type();
            AttackStatistics::new(damage, range, speed, damage_per_second, attack_type)
        });
        let hero = hero_attributes
            .zip(leveled.as_ref())
            .map(|(attributes, figures)| {
                let strength =
                    AttributeStatistic::new(figures.strength, attributes.strength_per_level());
                let agility =
                    AttributeStatistic::new(figures.agility, attributes.agility_per_level());
                let intelligence = AttributeStatistic::new(
                    figures.intelligence,
                    attributes.intelligence_per_level(),
                );
                let primary = attributes.primary();
                HeroStatistics::new(strength, agility, intelligence, primary)
            });
        let hit_points = HitPoints::new(hit_points_amount);
        let hit_points_regen = HitPointsRegen::new(hit_points_regen_rate, regen_type);
        let mana = Mana::new(mana_amount);
        let mana_regen = ManaRegen::new(mana_regen_rate);
        let armor = Armor::new(armor_amount);
        let effective_hit_points = EffectiveHitPoints::new(effective_hit_points_amount);
        let evasion = Evasion::new(evasion_chance);
        Self {
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
            armor,
            defense_type,
            effective_hit_points,
            evasion,
            attack,
            hero,
        }
    }

    pub fn hit_points(self) -> HitPoints {
        self.hit_points
    }

    pub fn hit_points_regen(self) -> HitPointsRegen {
        self.hit_points_regen
    }

    pub fn mana(self) -> Mana {
        self.mana
    }

    pub fn mana_regen(self) -> ManaRegen {
        self.mana_regen
    }

    pub fn armor(self) -> Armor {
        self.armor
    }

    pub fn defense_type(self) -> DefenseType {
        self.defense_type
    }

    pub fn effective_hit_points(self) -> EffectiveHitPoints {
        self.effective_hit_points
    }

    pub fn evasion(self) -> Evasion {
        self.evasion
    }

    pub fn attack(self) -> Option<AttackStatistics> {
        self.attack
    }

    pub fn hero(self) -> Option<HeroStatistics> {
        self.hero
    }

    /// Effective hit points: raw hit points scaled by armor (each point ≈ +6% EHP)
    /// and evasion (survivability ×1/(1−evasion)), mirroring Warcraft III's model.
    fn effective_hit_points_from(hit_points: u32, armor: f32, evasion_chance: f32) -> f32 {
        let hit_points_float = AsFloat::from(hit_points).value;
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

    /// EHP factor from evasion, clamped below full dodge so the figure stays finite.
    fn evasion_multiplier(evasion_chance: f32) -> f32 {
        let evasion_ceiling: f32 = 0.95;
        let clamped_evasion = evasion_chance.clamp(0.0, evasion_ceiling);
        1.0 / (1.0 - clamped_evasion)
    }

    /// Mean damage per second over the attack cooldown. `None` when there is no real
    /// attack (a non-positive cooldown).
    fn damage_per_second(damage: DamageRange, cooldown_seconds: f32) -> Option<f32> {
        if cooldown_seconds <= 0.0 {
            return None;
        }
        let damage_minimum_float = AsFloat::from(damage.minimum()).value;
        let damage_maximum_float = AsFloat::from(damage.maximum()).value;
        let average_damage = (damage_minimum_float + damage_maximum_float) / 2.0;
        Some(average_damage / cooldown_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_per_second_is_mean_damage_over_cooldown() {
        let damage = DamageRange::new(24, 27);
        let damage_per_second = UnitStatistics::damage_per_second(damage, 1.35).unwrap();
        assert!((damage_per_second - 18.888).abs() < 0.01);
    }

    #[test]
    fn damage_per_second_is_none_without_a_real_attack() {
        let damage = DamageRange::new(0, 0);
        assert!(UnitStatistics::damage_per_second(damage, 0.0).is_none());
    }

    #[test]
    fn effective_hit_points_add_six_percent_per_armor_point() {
        let effective_hit_points = UnitStatistics::effective_hit_points_from(850, 3.0, 0.0);
        assert!((effective_hit_points - 1003.0).abs() < 0.5);
    }

    #[test]
    fn effective_hit_points_fold_in_evasion() {
        let effective_hit_points = UnitStatistics::effective_hit_points_from(1000, 0.0, 0.2);
        assert!((effective_hit_points - 1250.0).abs() < 0.5);
    }
}
