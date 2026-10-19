pub mod components;
mod data;
mod model;
mod presentation;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_footer::DialogFooterView;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::info_dialog_body::InfoDialogBodyView;
use dioxus::prelude::*;
use model::InfoDialogConfig;
use presentation::InfoDialogShell;
use tw_macro::assert_component;

#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    let InfoDialogShell {
        open,
        on_open_change,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
    } = InfoDialogShell::from(&props);
    let body = InfoDialogBodyView { intro, warning };
    let footer = DialogFooterView {
        label: primary_label,
        onclick: on_primary,
    };
    rsx! {
        WarcraftDialog::<InfoDialogBodyView,DialogFooterView> {
            title,
            body,
            footer,
            open,
            on_open_change,
        }
    }
}

assert_component!(InfoDialog);
