use super::components::active_mana_regen_gain::ActiveManaRegenGainProps;
use super::components::muted_mana_regen_gain::MutedManaRegenGainProps;
use super::props::ManaRegenGainProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;

impl From<&ManaRegenGainProps> for ActiveManaRegenGainProps {
    fn from(props: &ManaRegenGainProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}

impl From<&ManaRegenGainProps> for MutedManaRegenGainProps {
    fn from(props: &ManaRegenGainProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}
