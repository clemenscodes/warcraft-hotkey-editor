mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::BurgerMenuItemIconProps;

assert_component!(BurgerMenuItemIcon);

#[component]
pub fn BurgerMenuItemIcon(props: BurgerMenuItemIconProps) -> Element {
    let svg = props.svg;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: svg,
        }
    }
}
