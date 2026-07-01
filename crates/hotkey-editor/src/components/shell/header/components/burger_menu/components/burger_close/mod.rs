mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::BurgerCloseProps;
use style::CLASS;
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
