mod components;
mod props;
mod style;

use dioxus::prelude::*;

use style::COMMAND_GRID_SECTION_STYLES;

pub use components::{
    CommandGrid, CommandGridHeading, DragFollowerOverlay, GridTile, GridTileProps, GridTileState,
    HotkeyBadge, HotkeyBadgeProps, HotkeyBadgeState,
};
pub use props::CommandGridSectionProps;

#[component]
pub fn CommandGridSection(props: CommandGridSectionProps) -> Element {
    let heading = props.heading;
    rsx! {
        document::Stylesheet { href: COMMAND_GRID_SECTION_STYLES }
        div { class: "command-section",
            CommandGridHeading { heading }
            CommandGrid { ..props }
        }
    }
}
