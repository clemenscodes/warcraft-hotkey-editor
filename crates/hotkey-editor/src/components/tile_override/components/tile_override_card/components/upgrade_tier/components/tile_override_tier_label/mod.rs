mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideTierLabelProps;

assert_component!(TileOverrideTierLabel);

/// The "Level N of M" caption in the tier-cycling footer.
#[component]
pub fn TileOverrideTierLabel(props: TileOverrideTierLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
