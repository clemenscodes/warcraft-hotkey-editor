use dioxus::prelude::*;

use warcraft_keybinds::ResearchBehavior;

use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};

/// A research menu: positions and hotkeys live in the secondary namespace.
#[component]
pub fn ResearchGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<ResearchBehavior>::from(&props) }
    }
}
