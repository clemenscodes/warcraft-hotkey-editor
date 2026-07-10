pub mod components;
mod props;
mod view;

pub use view::UnitDetailRowView;
mod style;

use components::unit_command_grids::UnitCommandGrids;
use components::unit_override_panel::UnitOverridePanel;
use dioxus::prelude::*;
use props::UnitDetailRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The command grids beside the override panel (headed by "Hotkey override"). It owns
/// the override panel column directly, splitting its threaded domain data between the
/// grids and the override panel.
#[component]
pub fn UnitDetailRow(props: UnitDetailRowProps) -> Element {
    let grid_slots = props.grid_slots;
    let unit_id = grid_slots.unit_id;
    let command_card_slots = grid_slots.command_card_slots;
    let build_menu_slots = grid_slots.build_menu_slots;
    let uprooted_menu_slots = grid_slots.uprooted_menu_slots;
    let research_menu_slots = grid_slots.research_menu_slots;
    let override_target = props.override_target;
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
