pub mod components;
mod model;
mod view;

pub use view::GridLayoutEditorDialogBodyView;
mod style;

use components::grid_layout_editor_dialog_content::GridLayoutEditorDialogContent;
use dioxus::prelude::*;
use model::GridLayoutEditorDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorDialogBody(props: GridLayoutEditorDialogBodyModel) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    rsx! {
        div {
            class: CLASS,
            GridLayoutEditorDialogContent {
                cells,
                toggle_checked,
                on_toggle,
            }
        }
    }
}

assert_component!(GridLayoutEditorDialogBody);
