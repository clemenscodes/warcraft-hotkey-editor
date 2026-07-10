pub mod components;
mod props;
mod style;

use components::burger_close::{BurgerClose, BurgerCloseProps};
use dioxus::prelude::*;
pub use props::BurgerDrawerHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerHeader(props: BurgerDrawerHeaderProps) -> Element {
    let close = BurgerCloseProps::from(&props);
    rsx! {
        div { class: CLASS,
            BurgerClose { ..close }
        }
    }
}

assert_component!(BurgerDrawerHeader);
