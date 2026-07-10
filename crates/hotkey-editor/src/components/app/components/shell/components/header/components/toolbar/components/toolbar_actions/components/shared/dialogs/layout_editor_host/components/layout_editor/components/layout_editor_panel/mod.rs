pub mod components;
mod props;
mod style;

use components::layout_editor_body::{LayoutEditorBody, LayoutEditorBodyProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::LayoutEditorPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The layout editor's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn LayoutEditorPanel(props: LayoutEditorPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = LayoutEditorBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                LayoutEditorBody { ..body }
            }
        }
    }
}

assert_component!(LayoutEditorPanel);
