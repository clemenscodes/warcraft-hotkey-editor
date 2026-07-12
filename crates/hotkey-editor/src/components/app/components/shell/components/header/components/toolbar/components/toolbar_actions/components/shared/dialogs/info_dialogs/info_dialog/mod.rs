pub mod components;
mod data;
mod model;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::info_dialog_body::InfoDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::InfoDialogConfig;
use presentation::InfoDialogShell;
use style::CLASS;
use tw_macro::assert_component;

/// The shared shell for the download and import info dialogs: a centered instruction block
/// above a cancel/primary action row. Variants fill in the title, copy, warning, and
/// handlers via `InfoDialogConfig`; this renders the reusable `WarcraftDialog`, handing it
/// the isolated instruction-and-actions body region. The headless dialog owns the box and
/// the title/close header.
#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    use_body_scroll_lock(props.open);
    let InfoDialogShell {
        open,
        on_open_change,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    } = InfoDialogShell::from(&props);
    let body = InfoDialogBodyView {
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<InfoDialogBodyView, Empty> {
                title,
                body,
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(InfoDialog);
