use super::shared::grid_editor::{GridEditor, GridEditorView};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::CommandBehavior;

#[component]
pub fn CommandGridEditor(props: GridEditorView) -> Element {
    rsx! {
        GridEditor::<CommandBehavior> {
            config: props,
        }
    }
}

assert_component!(CommandGridEditor);
