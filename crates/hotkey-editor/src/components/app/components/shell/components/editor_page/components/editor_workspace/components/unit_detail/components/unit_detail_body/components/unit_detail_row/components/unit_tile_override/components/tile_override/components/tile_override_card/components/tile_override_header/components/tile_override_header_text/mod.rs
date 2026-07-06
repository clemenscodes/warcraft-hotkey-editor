pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::tile_override_id::TileOverrideId;
use components::tile_override_name::TileOverrideName;
use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverrideHeaderTextProps;

assert_component!(TileOverrideHeaderText);

/// The name-and-id column of the override panel header.
#[component]
pub fn TileOverrideHeaderText(props: TileOverrideHeaderTextProps) -> Element {
    rsx! {
        div { class: CLASS,
            TileOverrideName { ..props.name }
            TileOverrideId { ..props.id }
        }
    }
}
