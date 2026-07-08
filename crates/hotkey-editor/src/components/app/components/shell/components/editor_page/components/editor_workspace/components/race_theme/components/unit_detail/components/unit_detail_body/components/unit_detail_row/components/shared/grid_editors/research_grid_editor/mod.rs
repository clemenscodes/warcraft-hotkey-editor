use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};
use dioxus::prelude::*;
use warcraft_keybinds::ResearchBehavior;

/// A research menu: positions and hotkeys live in the secondary namespace.
use tw_macro::assert_component;
assert_component!(ResearchGridEditor);
#[component]
pub fn ResearchGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<ResearchBehavior>::from(&props) }
    }
}
