pub mod components;
mod data;
mod hooks;
mod logic;
mod props;
mod style;

use components::layout_editor_panel::LayoutEditorPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPicker, KeyPickerProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_layout_editor;
use logic::LayoutEditorShell;
pub use props::LayoutEditorProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(LayoutEditor);

/// The global hotkey layout editor. It owns its own dialog shell: the hook resolves
/// the grid cells, picker state, and handlers; the shell struct shapes the panel, and
/// this places the panel inside its own backdrop `div` (the dimmed, centring layer)
/// within the library `DialogRoot`. The nested key picker (a second modal) is shown
/// while a cell is being edited. The shell's `on_open_change` guards the close so
/// opening the nested picker does not dismiss the editor.
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let model = use_layout_editor(&props);
    use_body_scroll_lock(model.open);
    let picker_open = model.picker_open;
    let picker = KeyPickerProps::from(&model);
    let LayoutEditorShell {
        open,
        on_open_change,
        panel,
    } = LayoutEditorShell::from(&model);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                LayoutEditorPanel { ..panel }
            }
        }
        if picker_open {
            KeyPicker { ..picker }
        }
    }
}
