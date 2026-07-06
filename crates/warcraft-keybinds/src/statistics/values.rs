//! The typed scalar figures that make up a unit's combat statistics. Each wraps a
//! single primitive (or a small, inseparable pair) so the value carries its meaning
//! through the type system: a `HitPoints` can never be mistaken for a `Mana`, and no
//! caller can pass a raw `u32` where a stat is expected. Presentation — formatting a
//! figure into a display string, choosing a colour — is the renderer's job, never
//! the domain's; these types carry only the value.

use warcraft_api::{RegenType, UnitMeta, WarcraftObjectMeta};
use warcraft_database::ObjectLookup;

/// A unit's maximum hit points.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct HitPoints {
    value: u32,
}

impl HitPoints {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }
}

/// Hit-point regeneration per second, together with the condition under which it
/// applies (only at night, only on blight, or always).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct HitPointsRegen {
    value: f32,
    regen_type: RegenType,
}

impl HitPointsRegen {
    pub fn new(value: f32, regen_type: RegenType) -> Self {
        Self { value, regen_type }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    pub fn regen_type(self) -> RegenType {
        self.regen_type
    }

    /// Whether there is no regeneration (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}

/// A unit's maximum mana. Zero for a unit with no mana pool.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct Mana {
    value: u32,
}

impl Mana {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    /// Whether this unit has no mana pool (a mana of zero), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value == 0
    }
}

#[cfg(test)]
mod evasion_tests {
    use super::Evasion;
    use warcraft_api::WarcraftObjectMeta;
    use warcraft_database::ObjectLookup;

    fn unit_evasion(unit_id: &str) -> f32 {
        let object = ObjectLookup::by_id(unit_id).expect("unit exists in the database");
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("object is not a unit");
        };
        let evasion = Evasion::resolve(unit_meta);
        evasion.chance()
    }

    #[test]
    fn a_unit_without_an_evasion_ability_resolves_to_zero() {
        let footman_evasion = unit_evasion("hfoo");
        assert_eq!(footman_evasion, 0.0);
    }

    #[test]
    fn a_hero_with_evasion_resolves_a_positive_chance() {
        let demon_hunter_evasion = unit_evasion("Edem");
        assert!(demon_hunter_evasion > 0.0);
    }
}

/// Mana regeneration per second.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct ManaRegen {
    value: f32,
}

impl ManaRegen {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    /// Whether there is no regeneration (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}

/// A unit's armor. Fractional and can be negative (which amplifies incoming damage).
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Armor {
    value: f32,
}

impl Armor {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// Effective hit points: how much raw incoming damage the unit survives once its
/// hit points are scaled by armor and evasion. A derived figure, computed by
/// [`super::UnitStatistics`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct EffectiveHitPoints {
    value: f32,
}

impl EffectiveHitPoints {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// A unit's evasion chance, in the range `0.0..=1.0` (the fraction of attacks it
/// dodges). Zero for a unit with no evasion source.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Evasion {
    chance: f32,
}

impl Evasion {
    pub fn new(chance: f32) -> Self {
        Self { chance }
    }

    pub fn chance(self) -> f32 {
        self.chance
    }

    /// The evasion as a percentage in `0.0..=100.0` — the form the figure is shown in.
    pub fn percent(self) -> f32 {
        self.chance * 100.0
    }

    /// Whether the unit has no evasion source (a chance of zero), so no evasion row
    /// is shown.
    pub fn is_zero(self) -> bool {
        self.chance <= 0.0
    }

    /// The highest evasion chance a unit can field, across its standard and hero
    /// abilities at each ability's full level. Evasion abilities do not stack — the
    /// strongest wins — so this is the unit's dodge chance. `Evasion::default()` (a
    /// chance of zero) when the unit has no evasion source. This resolution scans the
    /// game database, so it is domain work and lives here, never in the renderer.
    pub fn resolve(unit_meta: &UnitMeta) -> Self {
        let standard_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();
        let ability_lists = [standard_abilities, hero_abilities];
        let mut best_chance: f32 = 0.0;
        for ability_list in ability_lists {
            for ability_id in ability_list {
                let ability_code = ability_id.value();
                let Some(ability_object) = ObjectLookup::by_id(ability_code) else {
                    continue;
                };
                let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
                    continue;
                };
                let evasion_chances = ability_meta.evasion_chances();
                for chance in evasion_chances {
                    if chance > best_chance {
                        best_chance = chance;
                    }
                }
            }
        }
        Self::new(best_chance)
    }
}

/// An attack's damage span, from minimum to maximum roll.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct DamageRange {
    minimum: u32,
    maximum: u32,
}

impl DamageRange {
    pub fn new(minimum: u32, maximum: u32) -> Self {
        Self { minimum, maximum }
    }

    pub fn minimum(self) -> u32 {
        self.minimum
    }

    pub fn maximum(self) -> u32 {
        self.maximum
    }
}

/// An attack's reach, in game distance units.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct AttackRange {
    value: u32,
}

impl AttackRange {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    /// Whether the attack is melee (a reach of zero), so no range row is shown.
    pub fn is_zero(self) -> bool {
        self.value == 0
    }
}

/// An attack's cooldown, in seconds between swings.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct AttackSpeed {
    cooldown_seconds: f32,
}

impl AttackSpeed {
    pub fn new(cooldown_seconds: f32) -> Self {
        Self { cooldown_seconds }
    }

    pub fn cooldown_seconds(self) -> f32 {
        self.cooldown_seconds
    }
}

/// Mean damage per second over the attack cooldown. A derived figure, computed by
/// [`super::UnitStatistics`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct DamagePerSecond {
    value: f32,
}

impl DamagePerSecond {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// A positive increment shown with a leading `+`: a regeneration rate (per second)
/// or a per-level attribute growth. The renderer renders it as `+{value}` and mutes
/// it when it is zero.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Gain {
    value: f32,
}

impl Gain {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }

    /// Whether there is no gain (zero or negative), so the figure renders muted.
    pub fn is_zero(self) -> bool {
        self.value <= 0.0
    }
}
