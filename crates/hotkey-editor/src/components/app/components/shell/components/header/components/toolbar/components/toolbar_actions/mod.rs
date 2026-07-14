pub mod components;
mod style;

use components::burger_menu::BurgerMenu;
use components::inline_actions::InlineActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToolbarActions() -> Element {
    rsx! {
        div {
            class: CLASS,
            InlineActions {



            }
            BurgerMenu {



            }
        }
    }
}

assert_component!(ToolbarActions);
