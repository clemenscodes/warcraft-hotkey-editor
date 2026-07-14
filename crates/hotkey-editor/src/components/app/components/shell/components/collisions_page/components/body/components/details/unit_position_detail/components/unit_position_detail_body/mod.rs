pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitPositionDetailBodyView;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_unit_position_detail::EmptyUnitPositionDetail;
use components::filled_unit_position_detail::FilledUnitPositionDetail;
use dioxus::prelude::*;
use model::UnitPositionDetailBodyModel;
use tw_macro::assert_component;

/// The per-unit position-collision detail card's body region. A dispatcher: when a unit is
/// selected it renders the filled pane (the unit header over its position-conflict cards),
/// otherwise the empty prompt. The selection is read from collision-selection context. It
/// renders no surface — the filled and empty panes carry their own inner layout, inside the
/// shared `DetailCard`.
#[component]
pub fn UnitPositionDetailBody(props: UnitPositionDetailBodyModel) -> Element {
    let selected_unit = use_collision_selection().selected_unit_position();
    if let Some(unit_view) = presentation::selected(&props, selected_unit) {
        rsx! {
            FilledUnitPositionDetail {
                unit_view,
            }
        }
    } else {
        rsx! {
            EmptyUnitPositionDetail {
                prompt: presentation::EMPTY_PROMPT,
            }
        }
    }
}

assert_component!(UnitPositionDetailBody);
