mod model;
mod view;

pub use view::AltStateControlsView;

use dioxus::prelude::*;
use tw_macro::assert_component;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::alt_state_position_button::AltStatePositionButton;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::override_key::OverrideKey;

use model::AltStateControlsModel;

/// The position button and off-state hotkey cell of the alt-state block; renders
/// nothing when the off-state is not editable in this context.
#[component]
pub fn AltStateControls(props: AltStateControlsModel) -> Element {
    let AltStateControlsModel {
        show,
        hotkey_label,
        is_editing,
        is_special,
        on_position_click,
        on_hotkey_activate,
    } = props;
    if !show {
        return rsx! {};
    }
    let position_title =
        String::from("Pick where the off-state button appears on the command card");
    let hotkey_title = String::from("Hotkey for the off state (writes Unhotkey)");
    rsx! {
        AltStatePositionButton {
            title: position_title,
            aria_label: "Edit off-state button position",
            on_click: on_position_click,
        }
        OverrideKey {
            label: hotkey_label,
            is_editing,
            is_special,
            title: hotkey_title,
            on_activate: on_hotkey_activate,
        }
    }
}

assert_component!(AltStateControls);
