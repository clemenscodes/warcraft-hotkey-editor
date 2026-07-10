pub mod components;
mod props;
mod style;

use components::help_dialog_body::{HelpDialogBody, HelpDialogBodyProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::HelpDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The help dialog's bordered box: it wraps the library `DialogContent` (focus trap and
/// dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn HelpDialogPanel(props: HelpDialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = HelpDialogBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                HelpDialogBody { ..body }
            }
        }
    }
}

assert_component!(HelpDialogPanel);
