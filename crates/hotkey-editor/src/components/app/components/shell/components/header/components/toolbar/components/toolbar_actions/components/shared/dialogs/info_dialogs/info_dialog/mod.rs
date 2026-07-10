pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use components::info_dialog_panel::InfoDialogPanel;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use logic::InfoDialogShell;
pub use props::InfoDialogConfig;
use style::CLASS;
use tw_macro::assert_component;

/// The shared shell for the download and import info dialogs: a centered
/// instruction block above a cancel/primary action row. Variants fill in the
/// title, copy, warning, and handlers via `InfoDialogConfig`; this owns its own
/// dialog shell — the shell struct shapes the panel, and this places the panel
/// inside its own backdrop `div` (the dimmed, centring layer) within the library
/// `DialogRoot`. No project class touches the library element.
#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    use_body_scroll_lock(props.open);
    let InfoDialogShell {
        open,
        on_open_change,
        panel,
    } = InfoDialogShell::from(&props);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                InfoDialogPanel { ..panel }
            }
        }
    }
}

assert_component!(InfoDialog);
