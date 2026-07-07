mod components;
mod data;
mod logic;
mod props;

use super::detail::{Detail, DetailBody, DetailContent};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{
    MiniGrid, MiniGridProps,
};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use logic::selected;
pub use props::IslandDetailProps;

/// The island (position-collision) detail extension: builds the mini-grid header and
/// per-unit conflict cards, fed into the base detail pane. Each conflict ability owns
/// and opens its own carriers dialog, so this pane knows nothing about it.
use tw_macro::assert_component;
assert_component!(IslandDetail);
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let Some(model) = selected(&props) else {
        let content = DetailContent::Empty(data::EMPTY_PROMPT);
        return rsx! {
            Detail { content }
        };
    };
    let coordinate = model.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    let header = rsx! {
        MiniGrid { ..mini_grid }
        RowMeta {
            Coordinate { coordinate }
            CollisionCount { count: model.count }
        }
    };
    let cards = rsx! {
        for card in model.cards {
            IslandConflictCard { ..card }
        }
    };
    let body = DetailBody::new(header, cards);
    let content = DetailContent::Loaded(body);
    rsx! {
        Detail { content }
    }
}
