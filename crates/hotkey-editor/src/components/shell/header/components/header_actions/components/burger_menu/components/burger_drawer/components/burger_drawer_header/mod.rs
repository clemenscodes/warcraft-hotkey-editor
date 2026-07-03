pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::burger_close::{BurgerClose, BurgerCloseProps};
use dioxus::prelude::*;
pub use props::BurgerDrawerHeaderProps;
use style::CLASS;
assert_component!(BurgerDrawerHeader);

#[component]
pub fn BurgerDrawerHeader(props: BurgerDrawerHeaderProps) -> Element {
    let close = BurgerCloseProps::from(&props);
    rsx! {
        div { class: CLASS,
            BurgerClose { ..close }
        }
    }
}
