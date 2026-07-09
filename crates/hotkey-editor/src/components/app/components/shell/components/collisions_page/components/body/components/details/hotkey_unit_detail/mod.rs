pub mod components;
mod data;
mod logic;
mod props;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_hotkey_unit_detail::EmptyHotkeyUnitDetail;
use components::filled_hotkey_unit_detail::{FilledHotkeyUnitDetail, FilledHotkeyUnitDetailProps};
use dioxus::prelude::*;
pub use props::HotkeyUnitDetailProps;
use tw_macro::assert_component;
assert_component!(HotkeyUnitDetail);

/// The shared-hotkey detail pane. A dispatcher: when a unit is selected it renders the
/// filled pane (the unit header over its conflict cards), otherwise the empty prompt.
/// The selection is read from collision-selection context.
#[component]
pub fn HotkeyUnitDetail(props: HotkeyUnitDetailProps) -> Element {
    let selected_unit = use_collision_selection().selected_hotkey_unit();
    if let Some(data) = logic::selected(&props, selected_unit) {
        let filled = FilledHotkeyUnitDetailProps::from(&data);
        rsx! {
            FilledHotkeyUnitDetail { ..filled }
        }
    } else {
        rsx! {
            EmptyHotkeyUnitDetail {}
        }
    }
}
