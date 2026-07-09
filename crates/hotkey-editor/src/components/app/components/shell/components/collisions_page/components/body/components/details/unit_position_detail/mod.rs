mod components;
mod data;
mod logic;
mod props;
mod style;

use components::unit_position_conflict_grid::{
    UnitPositionConflictGrid, UnitPositionConflictGridProps,
};
use components::unit_position_detail_header::{
    UnitPositionDetailHeader, UnitPositionDetailHeaderProps,
};
use dioxus::prelude::*;
use logic::UnitPositionDetailData;
pub use props::UnitPositionDetailProps;
use style::DETAIL;
use tw_macro::assert_component;
assert_component!(UnitPositionDetail);

/// The position-collision detail pane: the selected unit's header over its conflict
/// cards. It owns its own pane element directly; renders the empty prompt when nothing
/// is selected.
#[component]
pub fn UnitPositionDetail(props: UnitPositionDetailProps) -> Element {
    let Some(data) = logic::selected(&props) else {
        return rsx! {
            section {
                class: DETAIL,
                "data-empty": true,
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    let UnitPositionDetailData {
        unit,
        name,
        unit_id,
        count,
        cards,
    } = data;
    let header = UnitPositionDetailHeaderProps {
        unit,
        name,
        unit_id,
        count,
    };
    let grid = UnitPositionConflictGridProps { cards };
    rsx! {
        section {
            class: DETAIL,
            "data-empty": false,
            UnitPositionDetailHeader { ..header }
            UnitPositionConflictGrid { ..grid }
        }
    }
}
