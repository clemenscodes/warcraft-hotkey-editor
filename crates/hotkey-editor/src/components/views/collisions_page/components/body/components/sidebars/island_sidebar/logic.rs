use crate::components::views::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::views::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::views::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use crate::components::views::collisions_page::components::body::components::shared::row_meta::RowMeta;
use super::super::sidebar::components::collision_card::CollisionCardProps;
use super::props::IslandSidebarProps;
use dioxus::prelude::*;

/// One finished card per collision island, with its selected state, key, click
/// handler, and content (mini grid + coordinate + collision count).
pub(super) fn cards(props: &IslandSidebarProps) -> Vec<CollisionCardProps> {
    let mut selected_island = props.selected_island;
    let selected_key = selected_island.read().clone();
    props
        .islands
        .iter()
        .map(|island| {
            let collision_key = island.key().to_owned();
            let is_selected = selected_key.as_deref() == Some(island.key());
            let coordinate = island.coordinate();
            let collision_count = island.collision_count();
            let noun = if collision_count == 1 {
                "collision"
            } else {
                "collisions"
            };
            let count_text = format!("{collision_count} {noun}");
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_island.set(Some(key_for_click.clone()))
            });
            let grid = MiniGridProps { coordinate };
            let children = rsx! {
                MiniGrid { ..grid }
                RowMeta {
                    Coordinate { coordinate }
                    CollisionCount { text: count_text }
                }
            };
            CollisionCardProps {
                is_selected,
                collision_key,
                onclick,
                children,
            }
        })
        .collect()
}
