pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitDetailRowView;
mod style;

use components::unit_command_grids::UnitCommandGrids;
use components::unit_override_panel::UnitOverridePanel;
use dioxus::prelude::*;
use model::UnitDetailRowModel;
use presentation::UnitDetailRowPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// The command grids beside the override panel (headed by "Hotkey override"). It owns
/// the override panel column directly, splitting its threaded domain data between the
/// grids and the override panel.
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
            UnitOverridePanel { override_target }
        }
    }
}

assert_component!(UnitDetailRow);
