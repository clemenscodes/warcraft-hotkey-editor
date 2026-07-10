use super::grid_editor::{GridEditor, GridEditorView};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::CommandBehavior;

/// The ordinary command card, build menus, and off-state position pickers.
#[component]
pub fn CommandGridEditor(props: GridEditorView) -> Element {
    rsx! {
        GridEditor::<CommandBehavior> { config: props }
    }
}

assert_component!(CommandGridEditor);
