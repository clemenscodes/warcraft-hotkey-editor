pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::LayoutEditorView;
mod style;

use components::layout_editor_panel::LayoutEditorPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPicker;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use presentation::use_layout_editor;
use model::LayoutEditorModel;
use style::CLASS;
use tw_macro::assert_component;

/// The global hotkey layout editor. It owns its own dialog shell: the hook resolves
/// the grid cells, picker state, and handlers; this places the panel inside its own
/// backdrop `div` (the dimmed, centring layer) within the library `DialogRoot`. The
/// nested key picker (a second modal) is shown while a cell is being edited. The
/// `on_open_change` guard makes opening the nested picker not dismiss the editor.
#[component]
pub fn LayoutEditor(props: LayoutEditorModel) -> Element {
    let model = use_layout_editor(&props);
    let open_signal = model.open;
    use_body_scroll_lock(open_signal);
    let is_open = *open_signal.read();
    if !is_open {
        return rsx! {};
    }
    let on_open_change = model.on_open_change;
    let title = model.title;
    let on_close = model.on_close;
    let cells = model.cells;
    let toggle_checked = model.toggle_checked;
    let on_toggle = model.on_toggle;
    let on_apply = model.on_apply;
    let picker_open = model.picker_open;
    let picker_title = model.picker_title;
    let picker_rows = model.picker_rows;
    let picker_allow_conflict_pick = model.picker_allow_conflict_pick;
    let on_pick = model.on_pick;
    let on_picker_close = model.on_picker_close;
    rsx! {
        DialogRoot {
            open: is_open,
            on_open_change,
            div {
                class: CLASS,
                LayoutEditorPanel {
                    title,
                    on_close,
                    cells,
                    toggle_checked,
                    on_toggle,
                    on_apply,
                }
            }
        }
        if picker_open {
            KeyPicker {
                title: picker_title,
                rows: picker_rows,
                open: picker_open,
                allow_conflict_pick: picker_allow_conflict_pick,
                on_pick,
                on_close: on_picker_close,
            }
        }
    }
}

assert_component!(LayoutEditor);
