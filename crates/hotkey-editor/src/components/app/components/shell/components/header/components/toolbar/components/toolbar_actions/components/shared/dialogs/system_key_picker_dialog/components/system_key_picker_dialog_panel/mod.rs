pub mod components;
mod props;
mod style;

use components::system_key_picker_dialog_body::{
    SystemKeyPickerDialogBody, SystemKeyPickerDialogBodyProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::SystemKeyPickerDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The system key picker's bordered box: it wraps the library `DialogContent` (focus
/// trap and dialog semantics) and styles a real `div` of its own with the box `CLASS`,
/// so no project class ever lands on the library element. Holds the header row above the
/// scrolling board body.
#[component]
pub fn SystemKeyPickerDialogPanel(props: SystemKeyPickerDialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = SystemKeyPickerDialogBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                SystemKeyPickerDialogBody { ..body }
            }
        }
    }
}

assert_component!(SystemKeyPickerDialogPanel);
