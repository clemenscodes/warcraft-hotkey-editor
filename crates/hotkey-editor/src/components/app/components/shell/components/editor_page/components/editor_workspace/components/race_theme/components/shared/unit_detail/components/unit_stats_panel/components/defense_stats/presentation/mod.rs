use super::model::DefenseStatsModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_api::{ArmorFigure as Armor, EffectiveHitPoints, Evasion};

pub(super) struct DefenseFigures {
    pub(super) icon_src: Asset,
    pub(super) icon_alt: String,
    pub(super) armor: Armor,
    pub(super) defense_type: DefenseType,
    pub(super) effective_hit_points: EffectiveHitPoints,
    pub(super) evasion: Evasion,
}

impl From<&DefenseStatsModel> for DefenseFigures {
    fn from(props: &DefenseStatsModel) -> Self {
        let armor = props.armor;
        let defense_type = props.defense_type;
        let effective_hit_points = props.effective_hit_points;
        let evasion = props.evasion;
        let defense_icon_kind = StatIcon::from(defense_type);
        let icon_src = defense_icon_kind.asset();
        let defense_label = defense_type.to_string();
        let icon_alt = format!("{defense_label} defense icon");
        Self {
            icon_src,
            icon_alt,
            armor,
            defense_type,
            effective_hit_points,
            evasion,
        }
    }
}

impl ddd::Presentation for DefenseFigures {
    type Model = DefenseStatsModel;
}
