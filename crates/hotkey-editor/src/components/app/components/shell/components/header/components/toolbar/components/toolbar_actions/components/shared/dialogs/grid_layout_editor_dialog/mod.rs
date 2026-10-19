pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::GridLayoutEditorDialogView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_footer::DialogFooterView;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::grid_layout_editor_dialog_body::GridLayoutEditorDialogBodyView;
use dioxus::prelude::*;
use model::GridLayoutEditorDialogModel;
use presentation::GridLayoutEditorDialogPresentation;
use presentation::use_grid_layout_editor_dialog;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorDialog(props: GridLayoutEditorDialogModel) -> Element {
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
    } = use_grid_layout_editor_dialog(&props);
    let body = GridLayoutEditorDialogBodyView {
        cells,
        toggle_checked,
        on_toggle,
    };
    let footer = DialogFooterView {
        label: data::APPLY_LABEL,
        onclick: on_apply,
    };
    rsx! {
        WarcraftDialog::<GridLayoutEditorDialogBodyView,DialogFooterView> {
            title,
            body,
            footer,
            open: is_open,
            on_open_change,
        }
        if is_open {
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

assert_component!(GridLayoutEditorDialog);
