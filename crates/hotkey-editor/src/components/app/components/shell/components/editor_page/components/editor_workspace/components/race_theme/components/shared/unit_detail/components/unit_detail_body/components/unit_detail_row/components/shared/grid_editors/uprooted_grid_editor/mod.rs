use super::shared::grid_editor::{GridEditor, GridEditorView};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::AlternateFormBehavior;

/// An alternate-form menu (an uprooted Ancient).
#[component]
pub fn UprootedGridEditor(props: GridEditorView) -> Element {
    rsx! {
        GridEditor::<AlternateFormBehavior> { config: props }
    }
}

assert_component!(UprootedGridEditor);
