use dioxus::prelude::*;
use warcraft_api::Race;

/// The race label: the display name to show and the race whose accent colors its
/// active state.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabLabelProps {
    pub race: Race,
    pub label: String,
}
