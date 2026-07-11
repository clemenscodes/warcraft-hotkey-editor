mod model;
mod view;

pub use view::TileOverrideIdView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideIdModel;

/// The object id shown under the name in the override panel.
#[component]
pub fn TileOverrideId(props: TileOverrideIdModel) -> Element {
    rsx! {
        code { class: CLASS, {props.object_id.value()} }
    }
}

assert_component!(TileOverrideId);
