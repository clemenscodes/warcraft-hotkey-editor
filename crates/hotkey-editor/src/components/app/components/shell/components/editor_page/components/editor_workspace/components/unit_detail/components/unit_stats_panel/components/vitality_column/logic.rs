use super::super::shared::stat_row::StatRowProps;
use super::kinds::{HitPointsKind, HitPointsRegenKind, ManaKind, ManaRegenKind};
use super::props::VitalityColumnProps;

/// Every child's finished props for the vitality column, shaped out of the body: the
/// four value rows. This is the vitality column's counterpart to the combat column's
/// `CombatRows`.
pub(super) struct VitalityRows {
    pub(super) hit_points_row: StatRowProps<HitPointsKind>,
    pub(super) hit_points_regen_row: StatRowProps<HitPointsRegenKind>,
    pub(super) mana_row: StatRowProps<ManaKind>,
    pub(super) mana_regen_row: StatRowProps<ManaRegenKind>,
}

impl From<&VitalityColumnProps> for VitalityRows {
    fn from(props: &VitalityColumnProps) -> Self {
        let hit_points = props.hit_points;
        let hit_points_regen = props.hit_points_regen;
        let mana = props.mana;
        let mana_regen = props.mana_regen;
        let hit_points_row = StatRowProps::<HitPointsKind> { value: hit_points };
        let hit_points_regen_row = StatRowProps::<HitPointsRegenKind> {
            value: hit_points_regen,
        };
        let mana_row = StatRowProps::<ManaKind> { value: mana };
        let mana_regen_row = StatRowProps::<ManaRegenKind> { value: mana_regen };
        Self {
            hit_points_row,
            hit_points_regen_row,
            mana_row,
            mana_regen_row,
        }
    }
}
