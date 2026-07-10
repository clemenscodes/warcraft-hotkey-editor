mod props;
mod style;

use dioxus::prelude::*;
use props::BurgerMenuItemLabelProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerMenuItemLabel(props: BurgerMenuItemLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(BurgerMenuItemLabel);
