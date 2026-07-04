pub mod components;
mod hooks;
mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog_host::DownloadInfoDialogHost;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::{
    UploadInfoDialog, UploadInfoDialogProps,
};
use components::burger_backdrop::{BurgerBackdrop, BurgerBackdropProps};
use components::burger_drawer::BurgerDrawer;
use components::burger_toggle_icon::BurgerToggleIcon;
use dioxus::prelude::*;
use hooks::{BurgerMenuView, use_burger_menu};
use style::CLASS;
assert_component!(BurgerMenu);

/// The compact-layout menu: a hamburger button that opens a slide-in drawer with
/// every file action. Shown only in the compact header (the full header shows the
/// inline toolbar instead). It only shows and toggles the drawer of action rows —
/// each action sources its own state, and the download is owned by
/// `DownloadInfoDialogHost`, so the burger threads no document itself.
#[component]
pub fn BurgerMenu() -> Element {
    let view = use_burger_menu();
    let backdrop = BurgerBackdropProps {
        onclick: view.drawer.on_close,
    };
    let upload_dialog = UploadInfoDialogProps::from(&view);
    let BurgerMenuView {
        burger_open,
        download_info_open,
        toggle,
        drawer,
        ..
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
            BurgerBackdrop { ..backdrop }
            BurgerDrawer { ..drawer }
        }
        UploadInfoDialog { ..upload_dialog }
        DownloadInfoDialogHost { open: download_info_open }
    }
}
