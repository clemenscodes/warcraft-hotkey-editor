pub mod components;
mod model;
mod view;

pub use view::TileOverrideHeaderTextView;
mod style;

use dioxus::prelude::*;

use components::tile_override_id::TileOverrideId;
use components::tile_override_name::TileOverrideName;
use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideHeaderTextModel;

/// The name-and-id column of the override panel header.
#[component]
pub fn TileOverrideHeaderText(props: TileOverrideHeaderTextModel) -> Element {
    let TileOverrideHeaderTextModel {
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
