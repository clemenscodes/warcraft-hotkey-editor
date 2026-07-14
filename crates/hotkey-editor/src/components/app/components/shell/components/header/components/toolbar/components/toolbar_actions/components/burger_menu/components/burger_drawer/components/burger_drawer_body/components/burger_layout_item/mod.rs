mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::GridLayoutEditorDialog;
use dioxus::prelude::*;
use presentation::{BurgerLayoutItemPresentation, use_burger_layout_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerLayoutItem() -> Element {
    let BurgerLayoutItemPresentation {
        icon,
        label,
        state,
        aria_haspopup,
        aria_expanded,
        aria_label,
        open,
        onclick,
        on_open_change,
    } = use_burger_layout_item();
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role: None,
                aria_haspopup,
                aria_expanded,
                aria_pressed: None,
                aria_label,
                onclick,
            }
            GridLayoutEditorDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerLayoutItem);
