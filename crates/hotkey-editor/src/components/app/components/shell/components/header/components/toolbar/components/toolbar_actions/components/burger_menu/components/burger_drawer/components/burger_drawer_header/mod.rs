pub mod components;
mod data;
mod model;
mod view;

pub use view::BurgerDrawerHeaderView;
mod style;

use components::burger_close::BurgerClose;
use components::burger_drawer_title::BurgerDrawerTitle;
use dioxus::prelude::*;
use model::BurgerDrawerHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerHeader(props: BurgerDrawerHeaderModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            BurgerDrawerTitle {
                title: data::TITLE,
            }
            BurgerClose {
                onclick,
            }
        }
    }
}

assert_component!(BurgerDrawerHeader);
