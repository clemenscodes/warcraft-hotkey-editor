use super::shared::grid_editor::{GridEditor, GridEditorView};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::ResearchBehavior;

/// A research menu: positions and hotkeys live in the secondary namespace.
#[component]
pub fn ResearchGridEditor(props: GridEditorView) -> Element {
    rsx! {
        GridEditor::<ResearchBehavior> { config: props }
    }
}

assert_component!(ResearchGridEditor);
