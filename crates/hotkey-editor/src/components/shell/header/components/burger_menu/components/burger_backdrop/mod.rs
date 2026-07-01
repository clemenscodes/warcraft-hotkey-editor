mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::BurgerBackdropProps;
use style::CLASS;
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
