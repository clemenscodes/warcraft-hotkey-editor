mod model;
mod view;

pub use view::TileOverrideTierLabelView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideTierLabelModel;

/// The "Level N of M" caption in the tier-cycling footer.
#[component]
pub fn TileOverrideTierLabel(props: TileOverrideTierLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(TileOverrideTierLabel);
