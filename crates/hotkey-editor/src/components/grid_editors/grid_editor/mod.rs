mod components;
mod logic;
mod props;
mod style;

use dioxus::prelude::*;
use warcraft_keybinds::GridBehavior;

use components::{GridHeadingProps, GridProps};
use style::GRID_EDITOR_STYLES;

pub use components::{
    DragFollowerOverlay, Grid, GridHeading, GridTile, GridTileProps, GridTileState, GridTileView,
    HotkeyBadge, HotkeyBadgeProps, HotkeyBadgeState,
};
pub use props::{GridEditorConfig, GridEditorProps};

/// The grid editor: a labeled, editable grid of a unit's command slots. Generic
/// over the [`GridBehavior`] that decides how moves cascade; the three variant
/// wrappers bind it. Pure RSX — every child's props is a `From` conversion.
#[component]
pub(crate) fn GridEditor<B: GridBehavior>(props: GridEditorProps<B>) -> Element {
    rsx! {
        document::Stylesheet { href: GRID_EDITOR_STYLES }
        div { class: "grid-editor", "data-grid-id": props.config.heading,
            GridHeading { ..GridHeadingProps::from(&props) }
            Grid { ..GridProps::from(&props) }
        }
    }
}
