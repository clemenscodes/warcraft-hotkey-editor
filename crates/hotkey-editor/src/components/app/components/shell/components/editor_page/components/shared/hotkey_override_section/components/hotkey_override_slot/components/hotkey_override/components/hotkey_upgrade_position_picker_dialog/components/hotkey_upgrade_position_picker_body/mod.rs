mod model;
mod presentation;
mod view;

pub use view::HotkeyUpgradePositionPickerBodyView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::shared::hotkey_alt_position_picker_body::HotkeyAltPositionPickerBody;
use dioxus::prelude::*;
use model::HotkeyUpgradePositionPickerBodyModel;
use presentation::HotkeyUpgradePositionPickerBodyPresentation;
use presentation::use_hotkey_upgrade_position_picker_body;
use style::CLASS;
use tw_macro::assert_component;

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
