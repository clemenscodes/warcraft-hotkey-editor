mod props;
mod style;

use super::burger_menu_group::{BurgerMenuGroup, BurgerMenuGroupProps};
use super::burger_menu_item::BurgerMenuItem;
use crate::assert_component;
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
