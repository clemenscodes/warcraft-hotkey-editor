pub mod components;
mod presentation;
mod style;

use components::burger_backdrop::BurgerBackdrop;
use components::burger_drawer::BurgerDrawer;
use components::burger_onboarding::BurgerOnboarding;
use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_icon::ToolbarButtonIcon;
use crate::components::app::components::shell::components::shared::icons::ICON_BURGER;
use dioxus::prelude::*;
use presentation::{BurgerMenuView, use_burger_menu};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerMenu() -> Element {
    let view = use_burger_menu();
    let BurgerMenuView {
        is_open,
        toggle,
        on_close,
        on_close_click,
    } = view;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{is_open}",
            aria_controls: "burger-drawer",
            onclick: toggle,
            ToolbarButtonIcon {
                icon: ICON_BURGER,
            }
        }
        BurgerOnboarding {}
        if is_open {
            BurgerBackdrop {
                onclick: on_close_click,
            }
            BurgerDrawer {
                on_close,
            }
        }
    }
}

assert_component!(BurgerMenu);
