pub mod components;
mod model;
mod view;

pub use view::UnitPositionDetailView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_card::DetailCard;
use components::unit_position_detail_body::UnitPositionDetailBodyView;
use dioxus::prelude::*;
use model::UnitPositionDetailModel;
use tw_macro::assert_component;

#[component]
pub fn UnitPositionDetail(props: UnitPositionDetailModel) -> Element {
    let units = props.units;
    let body = UnitPositionDetailBodyView { units };
    rsx! {
        DetailCard::<UnitPositionDetailBodyView> {
            body,
        }
    }
}

assert_component!(UnitPositionDetail);
