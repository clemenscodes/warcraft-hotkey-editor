pub mod components;
mod data;
mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPicker, KeyPickerProps};
use dioxus::prelude::*;
use hooks::use_layout_editor;
pub use props::LayoutEditorProps;

/// The global hotkey layout editor. A variant of the `Dialog` base: the hook
/// resolves the grid cells, picker state, and handlers; the body composes the
/// shell with the centered content and the apply action, plus the nested key
/// picker shown while a cell is being edited.
use tw_macro::assert_component;
assert_component!(LayoutEditor);
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let model = use_layout_editor(&props);
    let open = model.open;
    if !open() {
        return rsx! {};
    }
    rsx! {
        Dialog { ..DialogProps::from(&model) }
        if model.picker_open {
            KeyPicker { ..KeyPickerProps::from(&model) }
        }
    }
}
