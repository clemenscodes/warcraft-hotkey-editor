pub mod components;
mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerDialog;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::grid_layout_editor_dialog_body::GridLayoutEditorDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::GridLayoutEditorDialogPresentation;
use presentation::use_grid_layout_editor_dialog;
use style::CLASS;
use tw_macro::assert_component;

/// The global hotkey layout editor. The presentation resolves the grid cells, picker state,
/// and handlers; this renders the reusable `WarcraftDialog` with the isolated grid-editor
/// body region, alongside the nested key picker (a second modal shown while a cell is being
/// edited). The `on_open_change` guard makes opening the nested picker not dismiss the
/// editor.
#[component]
pub fn GridLayoutEditorDialog() -> Element {
    let GridLayoutEditorDialogPresentation {
        is_open,
        on_open_change,
        title,
        cells,
        toggle_checked,
        on_toggle,
        on_apply,
        picker_open,
        picker_title,
        picker_rows,
        picker_allow_conflict_pick,
        on_pick,
        on_picker_close,
    } = use_grid_layout_editor_dialog();
    let body = GridLayoutEditorDialogBodyView {
        cells,
        toggle_checked,
        on_toggle,
        on_apply,
    };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<GridLayoutEditorDialogBodyView, Empty> {
                title,
                body,
                open: is_open,
                on_open_change,
            }
            if picker_open {
                KeyPickerDialog {
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
}

assert_component!(GridLayoutEditorDialog);
