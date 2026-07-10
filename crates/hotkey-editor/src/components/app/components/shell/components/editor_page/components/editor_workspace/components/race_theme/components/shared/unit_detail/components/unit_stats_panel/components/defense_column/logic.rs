use super::super::shared::stat_icon_frame::StatIconFrameProps;
use super::props::DefenseColumnProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

/// The defense column's figures, shaped out of the body: the defense icon, the three
/// always-shown values, the guarded evasion, and the defense type that drives the
/// matchup grid.
pub(super) struct DefenseFigures {
    pub(super) defense_icon: StatIconFrameProps,
    pub(super) armor: Armor,
    pub(super) defense_type: DefenseType,
    pub(super) effective_hit_points: EffectiveHitPoints,
    pub(super) evasion: Evasion,
}

impl From<&DefenseColumnProps> for DefenseFigures {
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
        Self {
            defense_icon,
            armor,
            defense_type,
            effective_hit_points,
            evasion,
        }
    }
}
