mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileLabelProps;

assert_component!(TileLabel);

/// The centered text fallback shown when an occupied tile has no icon.
#[component]
pub fn TileLabel(props: TileLabelProps) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
