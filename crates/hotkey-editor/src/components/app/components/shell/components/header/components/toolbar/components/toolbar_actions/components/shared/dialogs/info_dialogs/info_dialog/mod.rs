pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use components::info_dialog_body::InfoDialogBody;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use logic::InfoDialogShell;
pub use props::InfoDialogConfig;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(InfoDialog);

/// The shared shell for the download and import info dialogs: a centered
/// instruction block above a cancel/primary action row. Variants fill in the
/// title, copy, warning, and handlers via `InfoDialogConfig`; this owns its own
/// dialog shell — the header and scroll body nest under it directly.
#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    use_body_scroll_lock(props.open);
    let InfoDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = InfoDialogShell::from(&props);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                InfoDialogBody { ..body }
            }
        }
    }
}
