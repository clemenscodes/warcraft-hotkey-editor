mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::BurgerMenuItemIconProps;
use style::CLASS;
assert_component!(BurgerMenuItemIcon);

#[component]
pub fn BurgerMenuItemIcon(props: BurgerMenuItemIconProps) -> Element {
    let svg = props.svg;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: svg }
    }
}
