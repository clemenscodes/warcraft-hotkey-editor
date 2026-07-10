use super::components::active_hit_points_regen_gain::ActiveHitPointsRegenGainProps;
use super::components::muted_hit_points_regen_gain::MutedHitPointsRegenGainProps;
use super::props::HitPointsRegenGainProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;

impl From<&HitPointsRegenGainProps> for ActiveHitPointsRegenGainProps {
    fn from(props: &HitPointsRegenGainProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}

impl From<&HitPointsRegenGainProps> for MutedHitPointsRegenGainProps {
    fn from(props: &HitPointsRegenGainProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}
