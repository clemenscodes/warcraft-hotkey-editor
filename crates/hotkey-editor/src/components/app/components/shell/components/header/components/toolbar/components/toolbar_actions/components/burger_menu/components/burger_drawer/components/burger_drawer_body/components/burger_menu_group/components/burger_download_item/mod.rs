mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use dioxus::prelude::*;
use presentation::{BurgerDownloadItemPresentation, use_burger_download_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDownloadItem() -> Element {
    let BurgerDownloadItemPresentation {
        hidden,
        icon,
        label,
        state,
        role,
        open,
        onclick,
        on_open_change,
    } = use_burger_download_item();
    if hidden {
        return rsx! {};
    }
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
                aria_pressed: None,
                aria_label: None,
                onclick,
            }
            DownloadInfoDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerDownloadItem);
