pub mod components;
mod model;
mod view;

pub use view::LayoutEditorBodyView;
mod style;

use components::layout_editor_content::LayoutEditorContent;
use dioxus::prelude::*;
use model::LayoutEditorBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The layout editor's scrolling content region between the header and the panel
/// edge, holding the centered editor column.
#[component]
pub fn LayoutEditorBody(props: LayoutEditorBodyModel) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    let on_apply = props.on_apply;
    rsx! {
        div {
            class: CLASS,
            LayoutEditorContent { cells, toggle_checked, on_toggle, on_apply }
        }
    }
}

assert_component!(LayoutEditorBody);
