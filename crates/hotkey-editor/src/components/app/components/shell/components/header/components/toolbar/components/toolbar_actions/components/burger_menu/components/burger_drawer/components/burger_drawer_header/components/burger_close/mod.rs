mod props;
mod style;

use dioxus::prelude::*;
use props::BurgerCloseProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(BurgerClose);
