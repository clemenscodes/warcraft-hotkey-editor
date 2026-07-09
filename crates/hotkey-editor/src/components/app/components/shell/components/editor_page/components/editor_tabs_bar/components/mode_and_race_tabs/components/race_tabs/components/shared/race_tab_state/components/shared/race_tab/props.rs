use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::components::race_tab_label::RaceTabLabelProps;
use dioxus::prelude::*;

/// The base race tab's props: the already-shaped label child props plus the
/// pointer/keyboard/mount handlers the `<button>` needs. The active and inactive
/// variants each build this and render `RaceTab`; carrying the label child props as
/// data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabProps {
    pub label: RaceTabLabelProps,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<Event<MountedData>>,
}
