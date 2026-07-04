mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideIdProps;

assert_component!(TileOverrideId);

/// The object id shown under the name in the override panel.
#[component]
pub fn TileOverrideId(props: TileOverrideIdProps) -> Element {
    let text = props.text;
    rsx! {
        code { class: CLASS, {text} }
    }
}
