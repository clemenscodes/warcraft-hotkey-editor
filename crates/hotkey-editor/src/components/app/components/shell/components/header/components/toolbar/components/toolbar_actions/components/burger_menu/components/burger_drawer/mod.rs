pub mod components;
mod model;
mod view;

pub use view::BurgerDrawerView;
mod style;

use components::burger_drawer_body::BurgerDrawerBody;
use components::burger_drawer_header::BurgerDrawerHeader;
use dioxus::prelude::*;
use model::BurgerDrawerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawer(props: BurgerDrawerModel) -> Element {
    let on_close = props.on_close;
    rsx! {
        aside {
            class: CLASS,
            id: "burger-drawer",
            role: "navigation",
            aria_label: "Menu",
            BurgerDrawerHeader {
                onclick: on_close,
            }
            BurgerDrawerBody {
                on_close,
            }
        }
    }
}

assert_component!(BurgerDrawer);
