mod model;
mod view;

pub use view::AltStateControlsView;

use crate::components::app::components::shell::components::shared::icons::ICON_CROSSHAIR;
use dioxus::prelude::*;
use tw_macro::assert_component;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::card_icon_button_host::CardIconButtonHost;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::override_key::OverrideKey;

use model::AltStateControlsModel;

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
        CardIconButtonHost {
                icon: ICON_CROSSHAIR,
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
