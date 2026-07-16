pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitDetailRowView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::HotkeyOverrideSection;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_command_grids::UnitCommandGrids;
use dioxus::prelude::*;
use model::UnitDetailRowModel;
use presentation::UnitDetailRowPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailRow(props: UnitDetailRowModel) -> Element {
    let UnitDetailRowPresentation {
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        override_target,
    } = UnitDetailRowPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitCommandGrids {
                unit_id,
                command_card_slots,
                build_menu_slots,
                uprooted_menu_slots,
                research_menu_slots,
            }
            HotkeyOverrideSection {
                override_target,
            }
        }
    }
}

assert_component!(UnitDetailRow);
