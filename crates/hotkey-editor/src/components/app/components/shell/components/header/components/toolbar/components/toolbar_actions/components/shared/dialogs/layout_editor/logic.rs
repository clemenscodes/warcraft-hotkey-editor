use super::components::apply_button::{ApplyButton, ApplyButtonProps};
use super::components::layout_editor_content::LayoutEditorContent;
use super::components::layout_grid::{LayoutGrid, LayoutGridProps};
use super::components::layout_intro::LayoutIntro;
use super::components::move_hotkey_toggle::{MoveHotkeyToggle, MoveHotkeyToggleProps};
use super::hooks::LayoutEditorModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerProps;
use dioxus::prelude::*;

impl From<&LayoutEditorModel> for DialogProps {
    fn from(model: &LayoutEditorModel) -> Self {
        let open = model.open;
        let title = String::from("Global Hotkey Layout");
        let apply = ApplyButtonProps {
            on_apply: model.on_apply,
        };
        let footer = Some(rsx! {
            ApplyButton { ..apply }
        });
        let grid = LayoutGridProps {
            cells: model.cells.clone(),
        };
        let toggle = MoveHotkeyToggleProps {
            checked: model.toggle_checked,
            on_toggle: model.on_toggle,
        };
        let children = rsx! {
            LayoutEditorContent {
                LayoutIntro {}
                LayoutGrid { ..grid }
                MoveHotkeyToggle { ..toggle }
            }
        };
        Self {
            open,
            title,
            children,
            footer,
            on_open_change: None,
        }
    }
}

impl From<&LayoutEditorModel> for KeyPickerProps {
    fn from(model: &LayoutEditorModel) -> Self {
        let title = String::from("Pick a grid key");
        let rows = model.picker_rows.clone();
        let open = true;
        let allow_conflict_pick = true;
        let on_pick = model.on_pick;
        let on_close = model.on_picker_close;
        Self {
            title,
            rows,
            open,
            allow_conflict_pick,
            on_pick,
            on_close,
        }
    }
}
