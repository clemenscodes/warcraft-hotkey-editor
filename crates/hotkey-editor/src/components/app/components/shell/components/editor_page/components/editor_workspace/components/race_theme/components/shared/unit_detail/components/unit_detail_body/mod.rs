pub mod components;
mod model;
mod view;

pub use view::UnitDetailBodyView;
mod style;

use components::unit_detail_row::UnitDetailRow;
use dioxus::prelude::*;
use model::UnitDetailBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The card body: the grids-and-override row.
#[component]
pub fn UnitDetailBody(props: UnitDetailBodyModel) -> Element {
    let grid_slots = props.grid_slots;
    let override_target = props.override_target;
    rsx! {
        div {
            class: CLASS,
            UnitDetailRow {
                grid_slots,
                override_target,
            }
        }
    }
}

assert_component!(UnitDetailBody);
