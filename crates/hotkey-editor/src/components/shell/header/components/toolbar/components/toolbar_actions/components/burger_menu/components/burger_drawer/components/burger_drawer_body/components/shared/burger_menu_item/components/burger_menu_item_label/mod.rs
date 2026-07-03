mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::BurgerMenuItemLabelProps;
use style::CLASS;
assert_component!(BurgerMenuItemLabel);

#[component]
pub fn BurgerMenuItemLabel(props: BurgerMenuItemLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
