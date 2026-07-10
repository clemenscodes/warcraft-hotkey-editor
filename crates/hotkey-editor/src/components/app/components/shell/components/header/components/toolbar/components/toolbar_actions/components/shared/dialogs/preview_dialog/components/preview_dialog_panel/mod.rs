pub mod components;
mod props;
mod style;

use components::preview_dialog_body::{PreviewDialogBody, PreviewDialogBodyProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::PreviewDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PreviewDialogPanel);

/// The preview dialog's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling textarea body.
#[component]
pub fn PreviewDialogPanel(props: PreviewDialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = PreviewDialogBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                PreviewDialogBody { ..body }
            }
        }
    }
}
