//! A unit's attack statistics: the figures shown in the combat column when the unit
//! can attack.

use super::values::{AttackRange, AttackSpeed, DamagePerSecond, DamageRange};
use warcraft_api::AttackType;

/// The complete attack profile of a unit that can attack. `damage_per_second` is
/// `None` when the attack has no real cooldown (so a rate is undefined).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct AttackStatistics {
    damage: DamageRange,
    range: AttackRange,
    speed: AttackSpeed,
    damage_per_second: Option<DamagePerSecond>,
    attack_type: AttackType,
}

impl AttackStatistics {
    pub fn new(
        damage: DamageRange,
        range: AttackRange,
        speed: AttackSpeed,
        damage_per_second: Option<DamagePerSecond>,
        attack_type: AttackType,
    ) -> Self {
        Self {
            damage,
            range,
            speed,
            damage_per_second,
            attack_type,
        }
    }

    pub fn damage(self) -> DamageRange {
        self.damage
    }

    pub fn range(self) -> AttackRange {
        self.range
    }

    pub fn speed(self) -> AttackSpeed {
        self.speed
    }

    pub fn damage_per_second(self) -> Option<DamagePerSecond> {
        self.damage_per_second
    }

    pub fn attack_type(self) -> AttackType {
        self.attack_type
    }
}
