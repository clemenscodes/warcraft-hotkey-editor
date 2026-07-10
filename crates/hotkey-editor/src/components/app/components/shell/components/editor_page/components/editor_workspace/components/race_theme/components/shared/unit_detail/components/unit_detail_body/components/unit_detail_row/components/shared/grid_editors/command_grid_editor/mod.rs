use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};
use dioxus::prelude::*;
use tw_macro::assert_component;
use warcraft_keybinds::CommandBehavior;

/// The ordinary command card, build menus, and off-state position pickers.
#[component]
pub fn CommandGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<CommandBehavior>::from(&props) }
    }
}

assert_component!(CommandGridEditor);
