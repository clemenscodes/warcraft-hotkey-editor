mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use super::burger_menu_group::{BurgerMenuGroup, BurgerMenuGroupProps};
use super::burger_menu_item::BurgerMenuItem;

pub use props::BurgerDrawerBodyProps;

assert_component!(BurgerDrawerBody);

#[component]
pub fn BurgerDrawerBody(props: BurgerDrawerBodyProps) -> Element {
    let group = BurgerMenuGroupProps::from(&props);
    let layout = props.layout;
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem { ..layout }
            BurgerMenuGroup { ..group }
        }
    }
}
