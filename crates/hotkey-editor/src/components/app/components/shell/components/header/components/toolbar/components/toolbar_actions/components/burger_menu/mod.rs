pub mod components;
mod data;
mod presentation;
mod state;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog_host::DownloadInfoDialogHost;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use components::burger_backdrop::BurgerBackdrop;
use components::burger_drawer::BurgerDrawer;
use components::burger_toggle_icon::BurgerToggleIcon;
use dioxus::prelude::*;
use presentation::{BurgerMenuView, use_burger_menu};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout menu: a hamburger button that opens a slide-in drawer with
/// every file action. Shown only in the compact header (the full header shows the
/// inline toolbar instead). It only shows and toggles the drawer of action rows —
/// each action sources its own state, and the download is owned by
/// `DownloadInfoDialogHost`, so the burger threads no document itself.
#[component]
pub fn BurgerMenu() -> Element {
    let view = use_burger_menu();
    let BurgerMenuView {
        burger_open,
        upload_info_open,
        download_info_open,
        toggle,
        on_close,
        layout,
        items,
    } = view;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{burger_open()}",
            aria_controls: "burger-drawer",
            onclick: toggle,
            BurgerToggleIcon {}
        }
        if burger_open() {
            BurgerBackdrop { onclick: on_close }
            BurgerDrawer {
                on_close,
                layout,
                items,
            }
        }
        UploadInfoDialog { open: upload_info_open }
        DownloadInfoDialogHost { open: download_info_open }
    }
}

assert_component!(BurgerMenu);
