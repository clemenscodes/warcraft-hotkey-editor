pub mod components;
mod model;
mod view;

pub use view::UnitPositionPagerCardHostView;
mod style;

use components::unit_position_pager_card::UnitPositionPagerCard;
use dioxus::prelude::*;
use model::UnitPositionPagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPositionPagerCardHost(props: UnitPositionPagerCardHostModel) -> Element {
    let unit = props.unit;
    rsx! {
        div {
            class: CLASS,
            UnitPositionPagerCard {
                unit,
            }
        }
    }
}

assert_component!(UnitPositionPagerCardHost);
