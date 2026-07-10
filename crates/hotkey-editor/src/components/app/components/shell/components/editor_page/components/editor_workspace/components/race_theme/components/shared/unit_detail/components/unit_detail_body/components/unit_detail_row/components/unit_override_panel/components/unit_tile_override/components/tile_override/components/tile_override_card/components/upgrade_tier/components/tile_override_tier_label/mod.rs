mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverrideTierLabelProps;

/// The "Level N of M" caption in the tier-cycling footer.
#[component]
pub fn TileOverrideTierLabel(props: TileOverrideTierLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(TileOverrideTierLabel);
