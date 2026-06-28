mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use crate::components::command_grid::components::{
    CommandGrid, CommandGridHeading, CommandGridHeadingProps, CommandGridProps,
};
use style::COMMAND_GRID_SECTION_STYLES;
use warcraft_keybinds::GridBehavior;

pub(crate) use props::GridSectionProps;

#[component]
pub(crate) fn GridSection<B: GridBehavior>(props: GridSectionProps<B>) -> Element {
    rsx! {
        document::Stylesheet { href: COMMAND_GRID_SECTION_STYLES }
        div {
            class: "grid-section",
            CommandGridHeading { ..CommandGridHeadingProps::from(&props) }
            CommandGrid { ..CommandGridProps::from(&props) }
        }
    }
}
