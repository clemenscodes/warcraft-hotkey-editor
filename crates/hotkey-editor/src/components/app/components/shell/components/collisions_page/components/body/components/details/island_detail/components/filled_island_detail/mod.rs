pub mod components;
mod props;
mod style;

use components::island_conflict_grid::{IslandConflictGrid, IslandConflictGridProps};
use components::island_detail_header::{IslandDetailHeader, IslandDetailHeaderProps};
use dioxus::prelude::*;
pub use props::FilledIslandDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated island detail pane: the island's mini-grid coordinate header over its
/// per-unit conflict cards.
#[component]
pub fn FilledIslandDetail(props: FilledIslandDetailProps) -> Element {
    let header = IslandDetailHeaderProps::from(&props);
    let grid = IslandConflictGridProps::from(&props);
    rsx! {
        section {
            class: CLASS,
            IslandDetailHeader { ..header }
            IslandConflictGrid { ..grid }
        }
    }
}

assert_component!(FilledIslandDetail);
