pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::burger_menu_group::{BurgerMenuGroup, BurgerMenuGroupProps};
use components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
pub use props::BurgerDrawerBodyProps;
use style::CLASS;
assert_component!(BurgerDrawerBody);

#[component]
pub fn BurgerDrawerBody(props: BurgerDrawerBodyProps) -> Element {
    let group = BurgerMenuGroupProps::from(&props);
    let layout = props.layout;
    rsx! {
        div { class: CLASS,
            BurgerMenuItem { ..layout }
            BurgerMenuGroup { ..group }
        }
    }
}
