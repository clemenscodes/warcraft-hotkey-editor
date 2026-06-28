use dioxus::prelude::*;

use crate::components::command_grid::CommandGridSectionProps;
use warcraft_keybinds::AlternateFormBehavior;

use super::section::{GridSection, GridSectionProps};

/// An alternate-form menu (an uprooted Ancient).
#[component]
pub fn UprootedGridSection(props: CommandGridSectionProps) -> Element {
    let grid_section_props = GridSectionProps::<AlternateFormBehavior>::new(props);
    rsx! {
        GridSection { ..grid_section_props }
    }
}
