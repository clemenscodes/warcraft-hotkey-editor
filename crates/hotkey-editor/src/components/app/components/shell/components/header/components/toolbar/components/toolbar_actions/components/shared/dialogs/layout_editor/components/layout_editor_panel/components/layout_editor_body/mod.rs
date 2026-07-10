pub mod components;
mod props;
mod style;

use components::layout_editor_content::{LayoutEditorContent, LayoutEditorContentProps};
use dioxus::prelude::*;
pub use props::LayoutEditorBodyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(LayoutEditorBody);

/// The layout editor's scrolling content region between the header and the panel
/// edge, holding the centered editor column.
#[component]
pub fn LayoutEditorBody(props: LayoutEditorBodyProps) -> Element {
    let content = LayoutEditorContentProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            LayoutEditorContent { ..content }
        }
    }
}
