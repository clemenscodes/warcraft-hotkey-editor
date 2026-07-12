pub mod components;
mod model;
mod view;

pub use view::GridLayoutEditorDialogPanelView;
mod style;

use components::grid_layout_editor_dialog_body::GridLayoutEditorDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use model::GridLayoutEditorDialogPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The layout editor's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn GridLayoutEditorDialogPanel(props: GridLayoutEditorDialogPanelModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    let on_apply = props.on_apply;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                GridLayoutEditorDialogBody { cells, toggle_checked, on_toggle, on_apply }
            }
        }
    }
}

assert_component!(GridLayoutEditorDialogPanel);
