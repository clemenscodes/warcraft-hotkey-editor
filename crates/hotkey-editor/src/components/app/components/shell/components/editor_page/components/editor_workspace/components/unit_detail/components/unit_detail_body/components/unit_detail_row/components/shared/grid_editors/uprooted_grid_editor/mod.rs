use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};
use dioxus::prelude::*;
use warcraft_keybinds::AlternateFormBehavior;

/// An alternate-form menu (an uprooted Ancient).
#[component]
pub fn UprootedGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<AlternateFormBehavior>::from(&props) }
    }
}
