pub mod components;
mod props;
mod view;

pub use view::KeyPickerPanelView;
mod style;

use components::key_picker_body::KeyPickerBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::KeyPickerPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The key picker's bordered box: it wraps the library `DialogContent` (focus trap and
/// dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling board body.
#[component]
pub fn KeyPickerPanel(props: KeyPickerPanelProps) -> Element {
    let title = props.title.clone();
    let on_close = props.on_close;
    let columns = props.columns.clone();
    let on_pick = props.on_pick;
    let on_board_close = props.on_board_close;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader {
                    title,
                    on_close,
                }
                KeyPickerBody {
                    columns,
                    on_pick,
                    on_close: on_board_close,
                }
            }
        }
    }
}

assert_component!(KeyPickerPanel);
