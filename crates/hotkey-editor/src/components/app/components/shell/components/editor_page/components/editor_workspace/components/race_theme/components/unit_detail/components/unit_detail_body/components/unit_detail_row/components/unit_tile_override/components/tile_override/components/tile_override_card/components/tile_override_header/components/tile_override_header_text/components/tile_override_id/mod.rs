mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverrideIdProps;

assert_component!(TileOverrideId);

/// The object id shown under the name in the override panel.
#[component]
pub fn TileOverrideId(props: TileOverrideIdProps) -> Element {
    rsx! {
        code { class: CLASS, {props.object_id.value()} }
    }
}
