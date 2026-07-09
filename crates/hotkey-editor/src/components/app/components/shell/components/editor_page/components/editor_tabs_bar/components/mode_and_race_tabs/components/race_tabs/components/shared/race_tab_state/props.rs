use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::RaceTabsProps;
use dioxus::prelude::*;
use warcraft_api::Race;

/// One race tab's state input: which race the tab is, plus the shared navigation it
/// compares against (to know whether it is the active tab) and writes on activation.
/// The race is the per-race wrapper's one discriminator; the navigation is forwarded
/// whole as a sub-field, so nothing is copied signal-by-signal.
#[derive(Props, Clone, Copy, PartialEq)]
pub struct RaceTabStateProps {
    pub race: Race,
    pub navigation: RaceTabsProps,
}
