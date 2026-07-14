pub mod components;
mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use components::upload_button_input::UploadButtonInput;
use dioxus::prelude::*;
use presentation::{UploadButtonPresentation, use_upload_button};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UploadButton() -> Element {
    let UploadButtonPresentation {
        icon,
        aria_label,
        open,
        onclick,
        on_open_change,
        on_change,
    } = use_upload_button();
    rsx! {
        div {
            class: CLASS,
            UploadButtonInput {
                on_change,
            }
            ToolbarButton {
                icon,
                aria_label,
                onclick,
            }
            UploadInfoDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(UploadButton);
