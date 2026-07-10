pub mod components;
mod props;
mod view;

pub use view::TileOverrideHeaderTextView;
mod style;

use dioxus::prelude::*;

use components::tile_override_id::TileOverrideId;
use components::tile_override_name::TileOverrideName;
use style::CLASS;
use tw_macro::assert_component;

use props::TileOverrideHeaderTextProps;

/// The name-and-id column of the override panel header.
#[component]
pub fn TileOverrideHeaderText(props: TileOverrideHeaderTextProps) -> Element {
    let TileOverrideHeaderTextProps {
        name_text,
        object_id,
    } = props;
    rsx! {
        div { class: CLASS,
            TileOverrideName { text: name_text }
            TileOverrideId { object_id }
        }
    }
}

assert_component!(TileOverrideHeaderText);
