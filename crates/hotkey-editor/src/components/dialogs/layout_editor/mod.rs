pub mod components;
mod data;
mod hooks;
mod props;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::shared::key_picker::KeyPicker;
use components::apply_button::ApplyButton;
use components::layout_editor_content::LayoutEditorContent;
use components::layout_grid::LayoutGrid;
use components::layout_intro::LayoutIntro;
use components::move_hotkey_toggle::MoveHotkeyToggle;
use hooks::use_layout_editor;

use super::dialog::Dialog;

pub use props::LayoutEditorProps;

assert_component!(LayoutEditor);

/// The global hotkey layout editor. A variant of the `Dialog` base: the hook
/// resolves the grid cells, picker state, and handlers; the body composes the
/// shell with the centered content and the apply action, plus the nested key
/// picker shown while a cell is being edited.
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let model = use_layout_editor(&props);
    let open = props.open;
    if !open() {
        return rsx! {};
    }
    rsx! {
        Dialog {
            open,
            title: "Global Hotkey Layout",
            on_open_change: Some(model.on_open_change),
            footer: Some(rsx! {
                ApplyButton { on_apply: model.on_apply }
            }),
            LayoutEditorContent {
                LayoutIntro {}
                LayoutGrid { cells: model.cells }
                MoveHotkeyToggle {
                    checked: model.toggle_checked,
                    on_toggle: model.on_toggle,
                }
            }
        }
        if model.picker_open {
            KeyPicker {
                title: "Pick a grid key".to_string(),
                rows: model.picker_rows,
                open: true,
                allow_conflict_pick: true,
                on_pick: model.on_pick,
                on_close: model.on_picker_close,
            }
        }
    }
}
