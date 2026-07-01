mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use super::burger_close::{BurgerClose, BurgerCloseProps};

pub use props::BurgerDrawerHeaderProps;

assert_component!(BurgerDrawerHeader);

#[component]
pub fn BurgerDrawerHeader(props: BurgerDrawerHeaderProps) -> Element {
    let close = BurgerCloseProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            BurgerClose { ..close }
        }
    }
}
