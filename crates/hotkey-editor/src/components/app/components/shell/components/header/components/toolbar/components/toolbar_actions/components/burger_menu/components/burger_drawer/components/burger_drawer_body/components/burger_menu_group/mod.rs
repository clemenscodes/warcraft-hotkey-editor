mod props;
mod style;

use super::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
pub use props::BurgerMenuGroupProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling list of file actions inside the drawer.
#[component]
pub fn BurgerMenuGroup(props: BurgerMenuGroupProps) -> Element {
    let items = props.items;
    rsx! {
        div { class: CLASS, role: "menu", aria_label: "File actions",
            for item in items.into_iter() {
                BurgerMenuItem { ..item }
            }
        }
    }
}

assert_component!(BurgerMenuGroup);
