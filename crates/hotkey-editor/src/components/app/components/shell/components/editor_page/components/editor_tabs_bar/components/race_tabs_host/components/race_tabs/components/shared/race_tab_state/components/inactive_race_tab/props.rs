use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::RaceTabProps;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use dioxus::prelude::*;

/// The inactive variant's props: the base tab's label and handlers, forwarded whole to
/// the `RaceTab` it renders. Inactive adds nothing on top — it is a named alias for the
/// base look, kept for symmetry with the active variant and for a clean dispatcher.
#[derive(Props, Clone, PartialEq)]
pub struct InactiveRaceTabProps {
    pub label: RaceTabLabelProps,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&InactiveRaceTabProps> for RaceTabProps {
    fn from(props: &InactiveRaceTabProps) -> Self {
        let label = props.label.clone();
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}
