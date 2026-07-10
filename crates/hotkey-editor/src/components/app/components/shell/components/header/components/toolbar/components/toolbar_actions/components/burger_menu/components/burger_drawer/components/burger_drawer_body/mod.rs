pub mod components;
mod props;
mod style;

use components::burger_menu_group::{BurgerMenuGroup, BurgerMenuGroupProps};
use components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
pub use props::BurgerDrawerBodyProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(BurgerDrawerBody);
