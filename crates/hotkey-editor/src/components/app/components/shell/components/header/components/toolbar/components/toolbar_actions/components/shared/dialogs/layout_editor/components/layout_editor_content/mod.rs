mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(LayoutEditorContent);

/// The centered body column of the layout editor: holds the intro, the grid, and
/// the move-hotkey toggle handed in as children.
#[component]
pub fn LayoutEditorContent(children: Element) -> Element {
    rsx! {
        div { class: CLASS, {children} }
    }
}
