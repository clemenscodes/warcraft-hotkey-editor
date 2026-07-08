mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use logic::IslandDetailData;
pub use props::IslandDetailProps;
use style::{DETAIL, GRID, HEADER};
use tw_macro::assert_component;
assert_component!(IslandDetail);

/// The position-island detail pane: the island's mini-grid and coordinate header over
/// its conflict cards. It owns its own pane, header, and grid elements directly;
/// renders the empty prompt when nothing is selected.
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
    let mini_grid = MiniGridProps { coordinate };
    rsx! {
        section {
            class: DETAIL,
            "data-empty": false,
            header {
                class: HEADER,
                MiniGrid { ..mini_grid }
                RowMeta {
                    Coordinate { coordinate }
                    CollisionCount { count }
                }
            }
            div {
                class: GRID,
                for card in cards {
                    IslandConflictCard { ..card }
                }
            }
        }
    }
}
