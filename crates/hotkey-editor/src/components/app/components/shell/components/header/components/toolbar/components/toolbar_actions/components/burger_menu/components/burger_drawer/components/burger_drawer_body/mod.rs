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

/// The drawer's scrolling content: the primary Grid Layout row on top, then the file-action
/// menu. The close handler flows through to the menu's non-dialog rows, which dismiss the drawer.
#[component]
pub fn BurgerDrawerBody(props: BurgerDrawerBodyModel) -> Element {
    let on_close = props.on_close;
    rsx! {
        div {
            class: CLASS,
            BurgerLayoutItem {
            


            }
            BurgerMenuGroup {
                on_close,
            }
        }
    }
}

assert_component!(BurgerDrawerBody);
