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
    let close_click = EventHandler::new(move |_event: MouseEvent| on_close.call(()));
    rsx! {
        aside {
            class: CLASS,
            id: "burger-drawer",
            role: "navigation",
            aria_label: "Menu",
            BurgerDrawerHeader {
                onclick: close_click,
            }
            BurgerDrawerBody {
                on_close,
            }
        }
    }
}

assert_component!(BurgerDrawer);
