pub mod components;
mod model;
mod view;

pub use view::BurgerDrawerBodyView;
mod style;

use components::burger_layout_item::BurgerLayoutItem;
use components::burger_menu_group::BurgerMenuGroup;
use dioxus::prelude::*;
use model::BurgerDrawerBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerBody(props: BurgerDrawerBodyModel) -> Element {
    let on_close = props.on_close;
    rsx! {
        div {
            class: CLASS,
            BurgerLayoutItem {}
            BurgerMenuGroup {
                on_close,
            }
        }
    }
}

assert_component!(BurgerDrawerBody);
