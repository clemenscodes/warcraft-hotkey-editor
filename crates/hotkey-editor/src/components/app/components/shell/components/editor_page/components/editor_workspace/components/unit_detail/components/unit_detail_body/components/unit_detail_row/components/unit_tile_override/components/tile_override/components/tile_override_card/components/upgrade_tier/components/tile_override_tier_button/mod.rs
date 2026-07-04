mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::TileOverrideTierButtonProps;

assert_component!(TileOverrideTierButton);

/// A prev/next arrow button in the tier-cycling footer.
#[component]
pub fn TileOverrideTierButton(props: TileOverrideTierButtonProps) -> Element {
    let aria_label = props.aria_label;
    let icon = props.icon;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            aria_label,
            onclick: on_click,
            span { aria_hidden: "true", dangerous_inner_html: icon }
        }
    }
}
