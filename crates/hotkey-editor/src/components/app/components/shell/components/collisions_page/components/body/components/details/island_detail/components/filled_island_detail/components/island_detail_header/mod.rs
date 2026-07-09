mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use components::island_conflict_meta::{IslandConflictMeta, IslandConflictMetaProps};
use dioxus::prelude::*;
pub use props::IslandDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IslandDetailHeader);

/// The island detail pane header row: the island's mini-grid beside its text meta
/// column.
#[component]
pub fn IslandDetailHeader(props: IslandDetailHeaderProps) -> Element {
    let coordinate = props.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    let meta = IslandConflictMetaProps::from(&props);
    rsx! {
        header {
            class: CLASS,
            MiniGrid { ..mini_grid }
            IslandConflictMeta { ..meta }
        }
    }
}
