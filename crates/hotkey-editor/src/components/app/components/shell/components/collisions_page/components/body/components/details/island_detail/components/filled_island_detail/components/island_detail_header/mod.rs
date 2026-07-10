mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use components::island_conflict_meta::IslandConflictMeta;
use dioxus::prelude::*;
use props::IslandDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

/// The island detail pane header row: the island's mini-grid beside its text meta
/// column.
#[component]
pub fn IslandDetailHeader(props: IslandDetailHeaderProps) -> Element {
    let coordinate = props.coordinate;
    let count = props.count;
    rsx! {
        header {
            class: CLASS,
            MiniGrid { coordinate }
            IslandConflictMeta { coordinate, count }
        }
    }
}

assert_component!(IslandDetailHeader);
