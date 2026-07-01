mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::BurgerMenuItemLabelProps;

assert_component!(BurgerMenuItemLabel);

#[component]
pub fn BurgerMenuItemLabel(props: BurgerMenuItemLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
