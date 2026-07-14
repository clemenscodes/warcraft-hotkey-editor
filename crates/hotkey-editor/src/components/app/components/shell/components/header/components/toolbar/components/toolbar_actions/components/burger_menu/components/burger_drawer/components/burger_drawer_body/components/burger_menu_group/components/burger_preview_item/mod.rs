mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::preview_dialog::PreviewDialog;
use dioxus::prelude::*;
use presentation::{BurgerPreviewItemPresentation, use_burger_preview_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerPreviewItem() -> Element {
    let BurgerPreviewItemPresentation {
        icon,
        label,
        state,
        role,
        aria_pressed,
        open,
        onclick,
        on_open_change,
    } = use_burger_preview_item();
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role,
                aria_haspopup: None,
                aria_expanded: None,
                aria_pressed,
                aria_label: None,
                onclick,
            }
            PreviewDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerPreviewItem);
