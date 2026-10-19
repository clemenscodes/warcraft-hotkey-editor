mod components;
mod model;
mod view;

pub use view::IslandDetailHeaderView;
mod style;

use components::island_conflict_meta::IslandConflictMeta;
use components::island_detail_mini_grid_slot::IslandDetailMiniGridSlot;
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
            IslandDetailMiniGridSlot {
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
