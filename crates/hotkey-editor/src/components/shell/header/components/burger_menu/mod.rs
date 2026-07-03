pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::info_dialogs::download_info_dialog::{
    DownloadInfoDialog, DownloadInfoDialogProps,
};
use crate::components::dialogs::info_dialogs::upload_info_dialog::{
    UploadInfoDialog, UploadInfoDialogProps,
};
use components::burger_backdrop::{BurgerBackdrop, BurgerBackdropProps};
use components::burger_drawer::BurgerDrawer;
use components::burger_toggle_icon::BurgerToggleIcon;
use dioxus::prelude::*;
use hooks::{BurgerMenuView, use_burger_menu};
pub use props::BurgerMenuProps;
use style::CLASS;
assert_component!(BurgerMenu);

/// The compact-layout menu: a hamburger button that opens a slide-in drawer with
/// every file action. Shown only in the compact header (the full header shows the
/// inline toolbar instead). Owns the drawer's open state through its composed hook.
#[component]
pub fn BurgerMenu(props: BurgerMenuProps) -> Element {
    let view = use_burger_menu(&props);
    let backdrop = BurgerBackdropProps {
        onclick: view.drawer.on_close,
    };
    let upload_dialog = UploadInfoDialogProps::from(&view);
    let download_dialog = DownloadInfoDialogProps::from(&view);
    let BurgerMenuView {
        burger_open,
        has_loaded_file,
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
        if has_loaded_file {
            DownloadInfoDialog { ..download_dialog }
        }
    }
}
