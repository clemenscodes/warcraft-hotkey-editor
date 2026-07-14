pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::presentation::{use_toolbar_actions, ToolbarActionKind};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use components::upload_button_input::UploadButtonInput;
use dioxus::prelude::*;
use presentation::use_upload_file_import;
use style::CLASS;
use tw_macro::assert_component;

/// The inline import control: a hidden file input plus the upload button. The button (icon,
/// label, click) comes from the shared toolbar-action set; clicking flips the shared
/// upload-info signal. This owns only the hidden input and its file-import handler.
#[component]
pub fn UploadButton() -> Element {
    let actions = use_toolbar_actions();
    let action = actions.get(ToolbarActionKind::Upload);
    let on_change = use_upload_file_import();
    rsx! {
        div {
            class: CLASS,
            UploadButtonInput { on_change }
            ToolbarButton {
                icon: action.icon,
                aria_label: action.aria_label,
                onclick: action.onclick,
            }
        }
    }
}

assert_component!(UploadButton);
