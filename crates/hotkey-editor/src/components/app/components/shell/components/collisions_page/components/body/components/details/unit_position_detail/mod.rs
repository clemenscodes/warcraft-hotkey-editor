pub mod components;
mod data;
mod logic;
mod props;
mod view;

pub use view::UnitPositionDetailView;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_unit_position_detail::EmptyUnitPositionDetail;
use components::filled_unit_position_detail::FilledUnitPositionDetail;
use dioxus::prelude::*;
use props::UnitPositionDetailProps;
use tw_macro::assert_component;

/// The position-collision detail pane. A dispatcher: when a unit is selected it renders
/// the filled pane (the unit header over its position-conflict cards), otherwise the
/// empty prompt. The selection is read from collision-selection context.
#[component]
pub fn UnitPositionDetail(props: UnitPositionDetailProps) -> Element {
    let selected_unit = use_collision_selection().selected_unit_position();
    if let Some(unit_view) = logic::selected(&props, selected_unit) {
        rsx! {
            FilledUnitPositionDetail { unit_view }
        }
    } else {
        rsx! {
            EmptyUnitPositionDetail {}
        }
    }
}

assert_component!(UnitPositionDetail);
