mod components;
mod model;
mod view;

pub use view::IslandDetailHeaderView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use components::island_conflict_meta::IslandConflictMeta;
use dioxus::prelude::*;
use model::IslandDetailHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandDetailHeader(props: IslandDetailHeaderModel) -> Element {
    let coordinate = props.coordinate;
    let count = props.count;
    rsx! {
        header {
            class: CLASS,
            MiniGrid {
                coordinate,
            }
            IslandConflictMeta {
                coordinate,
                count,
            }
        }
    }
}

assert_component!(IslandDetailHeader);
