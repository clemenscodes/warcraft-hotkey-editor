pub mod components;
mod props;
mod style;

use components::system_hotkeys_dialog_body::SystemHotkeysDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::SystemHotkeysDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The system-hotkeys dialog's bordered box: it wraps the library `DialogContent` (focus
/// trap and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so
/// no project class ever lands on the library element. Holds the header row above the
/// scrolling body, which reads its own state from context.
#[component]
pub fn SystemHotkeysDialogPanel(props: SystemHotkeysDialogPanelProps) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                SystemHotkeysDialogBody {}
            }
        }
    }
}

assert_component!(SystemHotkeysDialogPanel);
