mod model;
mod presentation;
mod view;

pub use view::HotkeyUpgradePositionPickerBodyView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::shared::hotkey_alt_position_picker_body::HotkeyAltPositionPickerBody;
use dioxus::prelude::*;
use model::HotkeyUpgradePositionPickerBodyModel;
use presentation::HotkeyUpgradePositionPickerBodyPresentation;
use presentation::use_hotkey_upgrade_position_picker_body;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form position picker's scrolling content region: the instruction
/// explainer above the embedded command grid where the upgraded-form button can be
/// dragged to a new cell. It builds the grid config from context in its own builder and
/// composes the shared `HotkeyAltPositionPickerBody`.
#[component]
pub fn HotkeyUpgradePositionPickerBody(props: HotkeyUpgradePositionPickerBodyModel) -> Element {
    let HotkeyUpgradePositionPickerBodyPresentation {
        explainer_text,
        grid_config,
    } = use_hotkey_upgrade_position_picker_body(&props);
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

assert_component!(HotkeyUpgradePositionPickerBody);
