use dioxus::prelude::*;

use crate::components::command_grid::CommandGridSectionProps;
use warcraft_keybinds::ResearchBehavior;

use super::section::{GridSection, GridSectionProps};

/// A research menu: positions and hotkeys live in the secondary namespace.
#[component]
pub fn ResearchGridSection(props: CommandGridSectionProps) -> Element {
    let grid_section_props = GridSectionProps::<ResearchBehavior>::new(props);
    rsx! {
        GridSection { ..grid_section_props }
    }
}
