use super::shared::grid_editor::{GridEditor, GridEditorView};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::AlternateFormBehavior;

#[component]
pub fn AlternateFormGridEditor(props: GridEditorView) -> Element {
    rsx! {
        GridEditor::<AlternateFormBehavior> {
            config: props,
        }
    }
}

assert_component!(AlternateFormGridEditor);
