mod model;
mod view;

pub use view::TileLabelView;
mod style;

use dioxus::prelude::*;
use model::TileLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The centered text fallback shown when an occupied tile has no icon.
#[component]
pub fn TileLabel(props: TileLabelModel) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(TileLabel);
