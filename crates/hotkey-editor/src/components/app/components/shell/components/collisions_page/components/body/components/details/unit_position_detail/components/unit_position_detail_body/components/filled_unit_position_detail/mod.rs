pub mod components;
mod model;
mod presentation;
mod view;

pub use view::FilledUnitPositionDetailView;
mod style;

use components::unit_position_conflict_grid::UnitPositionConflictGrid;
use components::unit_position_detail_header::UnitPositionDetailHeader;
use dioxus::prelude::*;
use model::FilledUnitPositionDetailModel;
use presentation::FilledUnitPositionDetailPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilledUnitPositionDetail(props: FilledUnitPositionDetailModel) -> Element {
    let FilledUnitPositionDetailPresentation {
        unit,
        count,
        unit_id,
        conflicts,
    } = FilledUnitPositionDetailPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitPositionDetailHeader {
                unit,
                count,
            }
            UnitPositionConflictGrid {
                conflicts,
                unit_id,
            }
        }
    }
}

assert_component!(FilledUnitPositionDetail);
