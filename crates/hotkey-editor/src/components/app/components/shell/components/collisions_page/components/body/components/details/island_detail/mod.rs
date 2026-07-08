mod components;
mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_grid::ConflictGrid;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail::Detail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_header::DetailHeader;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{
    MiniGrid, MiniGridProps,
};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use logic::IslandDetailData;
pub use props::IslandDetailProps;
use tw_macro::assert_component;

assert_component!(IslandDetail);

/// The island (position-collision) detail extension: resolves the selected island into
/// typed detail data and fills the base detail pane with its mini-grid header and
/// per-unit conflict cards.
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let Some(data) = logic::selected(&props) else {
        return rsx! {
            Detail {
                is_empty: true,
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
        Detail {
            DetailHeader {
                MiniGrid { ..mini_grid }
                RowMeta {
                    Coordinate { coordinate }
                    CollisionCount { count }
                }
            }
            ConflictGrid {
                for card in cards {
                    IslandConflictCard { ..card }
                }
            }
        }
    }
}
