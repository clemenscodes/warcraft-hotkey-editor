mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::TemplatesDialog;
use dioxus::prelude::*;
use presentation::{BurgerTemplatesItemPresentation, use_burger_templates_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerTemplatesItem() -> Element {
    let BurgerTemplatesItemPresentation {
        icon,
        label,
        state,
        role,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    } = use_burger_templates_item();
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed: None,
                aria_label: None,
                onclick,
            }
            TemplatesDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerTemplatesItem);
