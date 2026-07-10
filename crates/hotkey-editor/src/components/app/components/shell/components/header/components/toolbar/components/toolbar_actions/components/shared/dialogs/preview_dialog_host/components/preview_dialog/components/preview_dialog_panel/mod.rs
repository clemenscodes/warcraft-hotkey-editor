pub mod components;
mod props;
mod style;

use components::preview_dialog_body::PreviewDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::PreviewDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The preview dialog's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling textarea body.
#[component]
pub fn PreviewDialogPanel(props: PreviewDialogPanelProps) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let text = props.text;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                PreviewDialogBody { text }
            }
        }
    }
}

assert_component!(PreviewDialogPanel);
