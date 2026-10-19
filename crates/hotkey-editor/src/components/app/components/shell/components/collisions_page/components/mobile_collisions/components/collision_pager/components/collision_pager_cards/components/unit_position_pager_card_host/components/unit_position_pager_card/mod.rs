mod model;
mod view;

pub use view::UnitPositionPagerCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::components::unit_position_detail_body::components::filled_unit_position_detail::FilledUnitPositionDetail;
use dioxus::prelude::*;
use model::UnitPositionPagerCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPositionPagerCard(props: UnitPositionPagerCardModel) -> Element {
    let unit = props.unit;
    rsx! {
        div {
            class: CLASS,
            FilledUnitPositionDetail {
                unit_view: unit,
            }
        }
    }
}

assert_component!(UnitPositionPagerCard);
