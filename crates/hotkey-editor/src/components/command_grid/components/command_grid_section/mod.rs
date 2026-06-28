use dioxus::prelude::*;

use crate::components::command_grid::CommandGridSectionProps;
use warcraft_keybinds::CommandBehavior;

use super::section::{GridSection, GridSectionProps};

/// The ordinary command card, build menus, and off-state position pickers.
#[component]
pub fn CommandGridSection(props: CommandGridSectionProps) -> Element {
    rsx! {
        GridSection { ..GridSectionProps::<CommandBehavior>::from(&props) }
    }
}
