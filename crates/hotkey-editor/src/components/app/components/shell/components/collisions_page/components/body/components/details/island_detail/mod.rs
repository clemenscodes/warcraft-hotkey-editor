mod components;
mod data;
mod logic;
mod props;
mod style;

use components::island_conflict_grid::{IslandConflictGrid, IslandConflictGridProps};
use components::island_detail_header::{IslandDetailHeader, IslandDetailHeaderProps};
use dioxus::prelude::*;
use logic::IslandDetailData;
pub use props::IslandDetailProps;
use style::DETAIL;
use tw_macro::assert_component;
assert_component!(IslandDetail);

/// The position-island detail pane: the island's mini-grid and coordinate header over
/// its conflict cards. It owns its own pane element directly; renders the empty prompt
/// when nothing is selected.
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let Some(data) = logic::selected(&props) else {
        return rsx! {
            section {
                class: DETAIL,
                "data-empty": true,
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    let IslandDetailData {
        coordinate,
        count,
        cards,
    } = data;
    let header = IslandDetailHeaderProps { coordinate, count };
    let grid = IslandConflictGridProps { cards };
    rsx! {
        section {
            class: DETAIL,
            "data-empty": false,
            IslandDetailHeader { ..header }
            IslandConflictGrid { ..grid }
        }
    }
}
