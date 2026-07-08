use super::super::shared::stat_icon_frame::StatIconFrameProps;
use warcraft_api::AttackType;
use warcraft_keybinds::{AttackRange, AttackSpeed, AttackStatistics, DamagePerSecond, DamageRange};

/// The combat column's figures, shaped out of the body from the unit's attack: the
/// icon, the five stat values (three always shown, the guarded range and
/// damage-per-second), and the attack type that drives the matchup grid.
pub(super) struct CombatFigures {
    pub(super) icon: StatIconFrameProps,
    pub(super) damage: DamageRange,
    pub(super) range: AttackRange,
    pub(super) speed: AttackSpeed,
    pub(super) damage_per_second: Option<DamagePerSecond>,
    pub(super) attack_type: AttackType,
}

impl From<&AttackStatistics> for CombatFigures {
    fn from(attack: &AttackStatistics) -> Self {
        let icon = StatIconFrameProps::from(attack);
        let damage = attack.damage();
        let range = attack.range();
        let speed = attack.speed();
        let damage_per_second = attack.damage_per_second();
        let attack_type = attack.attack_type();
        Self {
            icon,
            damage,
            range,
            speed,
            damage_per_second,
            attack_type,
        }
    }
}
