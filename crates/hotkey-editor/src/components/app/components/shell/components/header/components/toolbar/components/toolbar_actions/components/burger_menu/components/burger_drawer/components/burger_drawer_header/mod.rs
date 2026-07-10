pub mod components;
mod props;
mod view;

pub use view::BurgerDrawerHeaderView;
mod style;

use components::burger_close::BurgerClose;
use dioxus::prelude::*;
use props::BurgerDrawerHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerHeader(props: BurgerDrawerHeaderProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        div { class: CLASS,
            BurgerClose { onclick }
        }
    }
}

assert_component!(BurgerDrawerHeader);
