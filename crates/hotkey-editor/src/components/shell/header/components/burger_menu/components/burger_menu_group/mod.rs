mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use super::burger_menu_item::BurgerMenuItem;

pub use props::BurgerMenuGroupProps;

assert_component!(BurgerMenuGroup);

/// The scrolling list of file actions inside the drawer.
#[component]
pub fn BurgerMenuGroup(props: BurgerMenuGroupProps) -> Element {
    let items = props.items;
    rsx! {
        div {
            class: CLASS,
            role: "menu",
            aria_label: "File actions",
            for item in items.into_iter() {
                BurgerMenuItem { ..item }
            }
        }
    }
}
