use dioxus::prelude::*;

use crate::components::command_grid::CommandGridSectionProps;
use warcraft_keybinds::AlternateFormBehavior;

use super::section::{GridSection, GridSectionProps};

/// An alternate-form menu (an uprooted Ancient).
#[component]
pub fn UprootedGridSection(props: CommandGridSectionProps) -> Element {
    rsx! {
        GridSection { ..GridSectionProps::<AlternateFormBehavior>::from(&props) }
    }
}
