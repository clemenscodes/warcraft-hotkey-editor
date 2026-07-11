pub mod components;
mod model;
mod view;

pub use view::SystemKeyPickerDialogPanelView;
mod style;

use components::system_key_picker_dialog_body::SystemKeyPickerDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use model::SystemKeyPickerDialogPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The system key picker's bordered box: it wraps the library `DialogContent` (focus
/// trap and dialog semantics) and styles a real `div` of its own with the box `CLASS`,
/// so no project class ever lands on the library element. Holds the header row above the
/// scrolling board body.
#[component]
pub fn SystemKeyPickerDialogPanel(props: SystemKeyPickerDialogPanelModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let columns = props.columns;
    let on_pick = props.on_pick;
    let board_on_close = props.board_on_close;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                SystemKeyPickerDialogBody {
                    columns,
                    on_pick,
                    on_close: board_on_close,
                }
            }
        }
    }
}

assert_component!(SystemKeyPickerDialogPanel);
