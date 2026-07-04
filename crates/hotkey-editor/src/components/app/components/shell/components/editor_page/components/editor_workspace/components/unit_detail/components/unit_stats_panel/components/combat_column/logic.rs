use super::super::shared::stat_icon_frame::StatIconFrameProps;
use super::super::shared::stat_row::StatRowProps;
use super::kinds::{AttackSpeedKind, AttackTypeKind, DamageKind};
use warcraft_api::AttackType;
use warcraft_keybinds::{AttackRange, AttackStatistics, DamagePerSecond};

/// Every child's finished props for the combat column, shaped out of the body: the
/// icon, the three value rows, and the guarded range and damage-per-second rows. This
/// is the combat column's counterpart to the grid editor tile's `EditorTileChrome`.
pub(super) struct CombatRows {
    pub(super) icon: StatIconFrameProps,
    pub(super) damage_row: StatRowProps<DamageKind>,
    pub(super) range: AttackRange,
    pub(super) speed_row: StatRowProps<AttackSpeedKind>,
    pub(super) damage_per_second: Option<DamagePerSecond>,
    pub(super) attack_type_row: StatRowProps<AttackTypeKind>,
    pub(super) attack_type: AttackType,
}

impl From<&AttackStatistics> for CombatRows {
    fn from(attack: &AttackStatistics) -> Self {
        let icon = StatIconFrameProps::from(attack);
        let attack_type = attack.attack_type();
        let damage = attack.damage();
        let range = attack.range();
        let speed = attack.speed();
        let damage_per_second = attack.damage_per_second();
        let damage_row = StatRowProps::<DamageKind> { value: damage };
        let speed_row = StatRowProps::<AttackSpeedKind> { value: speed };
        let attack_type_row = StatRowProps::<AttackTypeKind> { value: attack_type };
        Self {
            icon,
            damage_row,
            range,
            speed_row,
            damage_per_second,
            attack_type_row,
            attack_type,
        }
    }
}
