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

/// The layout editor's scrolling content region between the header and the panel
/// edge, holding the centered editor column.
#[component]
pub fn GridLayoutEditorDialogBody(props: GridLayoutEditorDialogBodyModel) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    let on_apply = props.on_apply;
    rsx! {
        div {
            class: CLASS,
            GridLayoutEditorDialogContent {
                cells,
                toggle_checked,
                on_toggle,
                on_apply,
            }
        }
    }
}

assert_component!(GridLayoutEditorDialogBody);
