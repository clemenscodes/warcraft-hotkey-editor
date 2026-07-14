pub mod components;
mod model;
mod view;

pub use view::UnitPositionDetailView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_card::DetailCard;
use components::unit_position_detail_body::UnitPositionDetailBodyView;
use dioxus::prelude::*;
use model::UnitPositionDetailModel;
use tw_macro::assert_component;

/// The position-collision detail pane. Composes the shared `DetailCard` surface, supplying
/// its body region — the dispatcher that shows the filled pane (the unit header over its
/// position-conflict cards) or the empty prompt.
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
