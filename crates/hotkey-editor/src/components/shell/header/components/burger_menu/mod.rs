pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::download_info_dialog::DownloadInfoDialog;
use crate::components::dialogs::upload_info_dialog::UploadInfoDialog;
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
    let BurgerMenuView {
        burger_open,
        upload_info_open,
        download_info_open,
        has_loaded_file,
        toggle,
        download_confirm,
        drawer,
    } = use_burger_menu(&props);
    let backdrop = BurgerBackdropProps {
        onclick: drawer.on_close,
    };
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
        UploadInfoDialog { open: upload_info_open }
        if has_loaded_file {
            DownloadInfoDialog { open: download_info_open, on_confirm: download_confirm }
        }
    }
}
