use super::super::shared::stat_icon_frame::StatIconFrameProps;
use super::super::shared::stat_row::StatRowProps;
use super::kinds::{ArmorKind, DefenseTypeKind, EffectiveHitPointsKind};
use super::props::DefenseColumnProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use warcraft_api::DefenseType;
use warcraft_keybinds::Evasion;

/// Every child's finished props for the defense column, shaped out of the body: the
/// defense icon, the three value rows, and the guarded evasion and matchup inputs.
/// This is the defense column's counterpart to the combat column's `CombatRows`.
pub(super) struct DefenseRows {
    pub(super) defense_icon: StatIconFrameProps,
    pub(super) armor_row: StatRowProps<ArmorKind>,
    pub(super) defense_type_row: StatRowProps<DefenseTypeKind>,
    pub(super) effective_hit_points_row: StatRowProps<EffectiveHitPointsKind>,
    pub(super) evasion: Evasion,
    pub(super) defense_type: DefenseType,
}

impl From<&DefenseColumnProps> for DefenseRows {
    fn from(props: &DefenseColumnProps) -> Self {
        let armor = props.armor;
        let defense_type = props.defense_type;
        let effective_hit_points = props.effective_hit_points;
        let evasion = props.evasion;
        let defense_icon_kind = StatIcon::from(defense_type);
        let defense_icon_source = defense_icon_kind.asset();
        let defense_label = defense_type.to_string();
        let defense_icon_alt = format!("{defense_label} defense icon");
        let defense_icon = StatIconFrameProps {
            src: defense_icon_source,
            alt: defense_icon_alt,
        };
        let armor_row = StatRowProps::<ArmorKind> { value: armor };
        let defense_type_row = StatRowProps::<DefenseTypeKind> {
            value: defense_type,
        };
        let effective_hit_points_row = StatRowProps::<EffectiveHitPointsKind> {
            value: effective_hit_points,
        };
        Self {
            defense_icon,
            armor_row,
            defense_type_row,
            effective_hit_points_row,
            evasion,
            defense_type,
        }
    }
}
