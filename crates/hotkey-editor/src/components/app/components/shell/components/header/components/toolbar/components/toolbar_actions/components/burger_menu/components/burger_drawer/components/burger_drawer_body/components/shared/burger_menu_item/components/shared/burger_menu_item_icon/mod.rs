mod props;
mod view;

pub use view::BurgerMenuItemIconView;
mod style;

use dioxus::prelude::*;
use props::BurgerMenuItemIconProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerMenuItemIcon(props: BurgerMenuItemIconProps) -> Element {
    let svg = props.svg;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: svg }
    }
}

assert_component!(BurgerMenuItemIcon);
