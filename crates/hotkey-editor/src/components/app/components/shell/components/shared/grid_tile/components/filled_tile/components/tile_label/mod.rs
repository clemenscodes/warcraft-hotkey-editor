mod props;
mod style;

use dioxus::prelude::*;
pub use props::TileLabelProps;
use style::CLASS;
use tw_macro::assert_component;
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
