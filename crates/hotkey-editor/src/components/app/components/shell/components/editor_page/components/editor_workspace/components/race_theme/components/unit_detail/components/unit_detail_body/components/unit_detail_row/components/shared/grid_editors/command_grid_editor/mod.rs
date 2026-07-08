use super::grid_editor::{GridEditor, GridEditorConfig, GridEditorProps};
use dioxus::prelude::*;
use warcraft_keybinds::CommandBehavior;

/// The ordinary command card, build menus, and off-state position pickers.
use tw_macro::assert_component;
assert_component!(CommandGridEditor);
#[component]
pub fn CommandGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<CommandBehavior>::from(&props) }
    }
}
