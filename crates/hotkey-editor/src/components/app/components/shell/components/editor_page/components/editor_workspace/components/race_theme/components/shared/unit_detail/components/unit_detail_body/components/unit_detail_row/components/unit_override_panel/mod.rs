pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::{
    GridHeading, GridHeadingProps,
};
use components::unit_tile_override::{UnitTileOverride, UnitTileOverrideProps};
use dioxus::prelude::*;
pub use props::UnitOverridePanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The override panel column: the "Hotkey override" heading above the override card.
/// Its own classed `aside`, sticky at the bottom on phones.
#[component]
pub fn UnitOverridePanel(props: UnitOverridePanelProps) -> Element {
    let heading = GridHeadingProps::from(&props);
    let tile_override = UnitTileOverrideProps::from(&props);
    rsx! {
        aside {
            class: CLASS,
            GridHeading { ..heading }
            UnitTileOverride { ..tile_override }
        }
    }
}

assert_component!(UnitOverridePanel);
