mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::BurgerCloseProps;

assert_component!(BurgerClose);

#[component]
pub fn BurgerClose(props: BurgerCloseProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Close menu",
            onclick,
            "\u{2715}"
        }
    }
}
