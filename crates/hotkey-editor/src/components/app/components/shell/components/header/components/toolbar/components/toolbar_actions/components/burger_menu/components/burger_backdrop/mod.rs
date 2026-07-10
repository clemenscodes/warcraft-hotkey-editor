mod props;
mod view;

pub use view::BurgerBackdropView;
mod style;

use dioxus::prelude::*;
use props::BurgerBackdropProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(BurgerBackdrop);
