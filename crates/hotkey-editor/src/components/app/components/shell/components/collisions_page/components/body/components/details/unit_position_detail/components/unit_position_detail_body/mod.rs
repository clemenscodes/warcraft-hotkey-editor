pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitPositionDetailBodyView;

use crate::services::collision_selection::context::use_collision_selection;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::empty_detail::EmptyDetail;
use components::filled_unit_position_detail::FilledUnitPositionDetail;
use dioxus::prelude::*;
use model::UnitPositionDetailBodyModel;
use tw_macro::assert_component;

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
            EmptyDetail {
                prompt: presentation::EMPTY_PROMPT,
            }
        }
    }
}

assert_component!(UnitPositionDetailBody);
