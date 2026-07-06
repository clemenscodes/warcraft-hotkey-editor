mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
