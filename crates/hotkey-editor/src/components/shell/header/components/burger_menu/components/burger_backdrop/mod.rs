mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::BurgerBackdropProps;

assert_component!(BurgerBackdrop);

/// The dimmed click-catcher behind the drawer; clicking it closes the menu.
#[component]
pub fn BurgerBackdrop(props: BurgerBackdropProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            role: "button",
            aria_label: "Close menu",
            tabindex: "-1",
            onclick,
        }
    }
}
