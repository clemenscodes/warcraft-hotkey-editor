mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use dioxus::prelude::*;
use presentation::{BurgerUploadItemPresentation, use_burger_upload_item};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout upload action: the drawer row that opens the upload info dialog, and the
/// dialog it owns, mounted beneath it. It owns the dialog's open signal locally, so the dialog
/// is part of the row and travels with it. Tapping it leaves the drawer open (closing the
/// drawer would unmount this row and its dialog), so the info dialog opens over the drawer. The
/// info dialog's confirm triggers the hidden file input (by element id) that lives in the
/// inline upload button, so this row mounts no input of its own.
#[component]
pub fn BurgerUploadItem() -> Element {
    let BurgerUploadItemPresentation {
        icon,
        label,
        state,
        role,
        open,
        onclick,
        on_open_change,
    } = use_burger_upload_item();
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
            UploadInfoDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerUploadItem);
