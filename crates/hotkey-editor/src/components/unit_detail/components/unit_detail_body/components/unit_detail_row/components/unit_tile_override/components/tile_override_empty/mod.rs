mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideEmptyProps;

assert_component!(TileOverrideEmpty);

/// The placeholder shown in the override panel before a grid tile is selected.
#[component]
pub fn TileOverrideEmpty(props: TileOverrideEmptyProps) -> Element {
    let message = props.message;
    rsx! {
        div { class: CLASS,
            p { {message} }
        }
    }
}
