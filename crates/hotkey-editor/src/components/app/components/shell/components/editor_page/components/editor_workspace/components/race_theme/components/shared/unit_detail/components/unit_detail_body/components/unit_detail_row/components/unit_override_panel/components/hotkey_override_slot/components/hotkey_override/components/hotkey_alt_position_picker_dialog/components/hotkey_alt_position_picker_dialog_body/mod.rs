mod model;
mod presentation;
mod view;

pub use view::HotkeyAltPositionPickerDialogBodyView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::shared::hotkey_alt_position_picker_body::HotkeyAltPositionPickerBody;
use dioxus::prelude::*;
use model::HotkeyAltPositionPickerDialogBodyModel;
use presentation::HotkeyAltPositionPickerDialogBodyPresentation;
use presentation::use_hotkey_alt_position_picker_dialog_body;
use style::CLASS;
use tw_macro::assert_component;

/// The off-state position picker dialog's body region: its presentation assembles the
/// embedded command grid's config from context and local picker signals, and this places
/// the shared position-picker scroll body — the instruction explainer above the grid —
/// inside the dialog content box.
#[component]
pub fn HotkeyAltPositionPickerDialogBody(props: HotkeyAltPositionPickerDialogBodyModel) -> Element {
    let HotkeyAltPositionPickerDialogBodyPresentation {
        explainer_text,
        grid_config,
    } = use_hotkey_alt_position_picker_dialog_body(&props);
    rsx! {
        div {
            class: CLASS,
            HotkeyAltPositionPickerBody {
                explainer_text,
                grid_config,
            }
        }
    }
}

assert_component!(HotkeyAltPositionPickerDialogBody);
