pub mod components;
mod props;
mod style;

use components::layout_editor_body::LayoutEditorBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::LayoutEditorPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The layout editor's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn LayoutEditorPanel(props: LayoutEditorPanelProps) -> Element {
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
                LayoutEditorBody { cells, toggle_checked, on_toggle, on_apply }
            }
        }
    }
}

assert_component!(LayoutEditorPanel);
