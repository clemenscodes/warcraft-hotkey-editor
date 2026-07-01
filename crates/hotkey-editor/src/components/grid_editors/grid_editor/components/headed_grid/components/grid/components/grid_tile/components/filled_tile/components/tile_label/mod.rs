mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::TileLabelProps;
use style::CLASS;
assert_component!(TileLabel);

/// The centered text fallback shown when an occupied tile has no icon.
#[component]
pub fn TileLabel(props: TileLabelProps) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        span { class: CLASS, {text} }
    }
}
