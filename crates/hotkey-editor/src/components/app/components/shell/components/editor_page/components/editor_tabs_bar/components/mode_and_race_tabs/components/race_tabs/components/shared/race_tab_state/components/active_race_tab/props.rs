use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::RaceTabProps;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use dioxus::prelude::*;

/// The active variant's props: the base tab's label and handlers, forwarded whole to the
/// `RaceTab` it composes. The active look is added on top (the accent overlay and the
/// `--label-color` its root publishes), never by changing these.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveRaceTabProps {
    pub label: RaceTabLabelProps,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<Event<MountedData>>,
}

impl From<&ActiveRaceTabProps> for RaceTabProps {
    fn from(props: &ActiveRaceTabProps) -> Self {
        let label = props.label.clone();
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        let onmounted = props.onmounted;
        Self {
            label,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}
