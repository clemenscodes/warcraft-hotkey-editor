pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HotkeyUnitDetailBodyView;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_hotkey_unit_detail::EmptyHotkeyUnitDetail;
use components::filled_hotkey_unit_detail::FilledHotkeyUnitDetail;
use dioxus::prelude::*;
use model::HotkeyUnitDetailBodyModel;
use tw_macro::assert_component;

/// The shared-hotkey detail card's body region. A dispatcher: when a unit is selected it
/// renders the filled pane (the unit header over its conflict cards), otherwise the empty
/// prompt. The selection is read from collision-selection context. It renders no surface —
/// the filled and empty panes carry their own inner layout, inside the shared `DetailCard`.
#[component]
pub fn HotkeyUnitDetailBody(props: HotkeyUnitDetailBodyModel) -> Element {
    let selected_unit = use_collision_selection().selected_hotkey_unit();
    if let Some(unit_view) = presentation::selected(&props, selected_unit) {
        rsx! {
            FilledHotkeyUnitDetail {
                unit_view,
            }
        }
    } else {
        rsx! {
            EmptyHotkeyUnitDetail {
                prompt: presentation::EMPTY_PROMPT,
            }
        }
    }
}

assert_component!(HotkeyUnitDetailBody);
