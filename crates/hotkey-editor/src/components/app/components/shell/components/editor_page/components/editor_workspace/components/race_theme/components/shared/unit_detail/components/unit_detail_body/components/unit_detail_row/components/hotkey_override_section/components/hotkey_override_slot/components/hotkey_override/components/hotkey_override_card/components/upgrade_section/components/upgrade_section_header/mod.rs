pub mod components;
mod model;
mod view;

pub use view::UpgradeSectionHeaderView;
mod style;

use components::upgrade_section_header_label_column::UpgradeSectionHeaderLabelColumn;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::alt_state_position_button_host::AltStatePositionButtonHost;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::override_key::OverrideKey;
use dioxus::prelude::*;
use model::UpgradeSectionHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UpgradeSectionHeader(props: UpgradeSectionHeaderModel) -> Element {
    let UpgradeSectionHeaderModel {
        hotkey_label,
        is_editing,
        is_special,
        on_position_click,
        on_hotkey_activate,
    } = props;
    let label_text = Some(String::from("Upgraded form"));
    let position_title =
        String::from("Pick where the upgraded-form button appears on the command card");
    let hotkey_title = String::from("Hotkey for the upgraded form");
    rsx! {
        div {
            class: CLASS,
            UpgradeSectionHeaderLabelColumn {
                text: label_text,
            }
            AltStatePositionButtonHost {
                title: position_title,
                aria_label: "Edit upgraded-form button position",
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
}

assert_component!(UpgradeSectionHeader);
