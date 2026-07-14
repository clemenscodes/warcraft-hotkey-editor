pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitDetailRowView;
mod style;

use components::hotkey_override_section::HotkeyOverrideSection;
use components::unit_command_grids::UnitCommandGrids;
use dioxus::prelude::*;
use model::UnitDetailRowModel;
use presentation::UnitDetailRowPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// The command grids above the hotkey-override section (headed by "Hotkey override"). It
/// owns that section directly, splitting its threaded domain data between the grids and
/// the hotkey-override section.
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
