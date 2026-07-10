pub mod components;
mod props;
mod style;

use components::island_conflict_grid::IslandConflictGrid;
use components::island_detail_header::IslandDetailHeader;
use dioxus::prelude::*;
use props::FilledIslandDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated island detail pane: the island's mini-grid coordinate header over its
/// per-unit conflict cards.
#[component]
pub fn FilledIslandDetail(props: FilledIslandDetailProps) -> Element {
    let coordinate = props.island.coordinate();
    let count = props.island.collision_count();
    let conflicts = props.island.conflicts().to_vec();
    rsx! {
        section {
            class: CLASS,
            IslandDetailHeader { coordinate, count }
            IslandConflictGrid { conflicts }
        }
    }
}

assert_component!(FilledIslandDetail);
