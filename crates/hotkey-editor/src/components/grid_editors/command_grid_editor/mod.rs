use dioxus::prelude::*;

use warcraft_keybinds::CommandBehavior;

use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};

/// The ordinary command card, build menus, and off-state position pickers.
#[component]
pub fn CommandGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<CommandBehavior>::from(&props) }
    }
}
