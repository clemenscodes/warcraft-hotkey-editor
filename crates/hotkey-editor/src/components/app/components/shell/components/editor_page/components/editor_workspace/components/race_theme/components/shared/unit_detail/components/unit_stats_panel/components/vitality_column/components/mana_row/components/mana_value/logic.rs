use super::components::active_mana_value::ActiveManaValueProps;
use super::components::muted_mana_value::MutedManaValueProps;
use super::props::ManaValueProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;

impl From<&ManaValueProps> for ActiveManaValueProps {
    fn from(props: &ManaValueProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}

impl From<&ManaValueProps> for MutedManaValueProps {
    fn from(props: &ManaValueProps) -> Self {
        let text = props.value.display();
        Self { text }
    }
}
