pub mod components;
mod data;
mod props;
mod view;

pub use view::UnitOverridePanelView;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::unit_tile_override::UnitTileOverride;
use data::HEADING;
use dioxus::prelude::*;
use props::UnitOverridePanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The override panel column: the "Hotkey override" heading above the override card.
/// Its own classed `aside`, sticky at the bottom on phones.
#[component]
pub fn UnitOverridePanel(props: UnitOverridePanelProps) -> Element {
    let override_target = props.override_target;
    let detail = override_target.detail;
    let active_container_slots = override_target.active_container_slots;
    rsx! {
        aside {
            class: CLASS,
            GridHeading { heading: HEADING }
            UnitTileOverride { detail, active_container_slots }
        }
    }
}

assert_component!(UnitOverridePanel);
