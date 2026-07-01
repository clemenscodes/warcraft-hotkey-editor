use super::super::collision_card::CollisionCardProps;
use super::super::island_collision_count::IslandCollisionCount;
use super::super::island_coord::IslandCoord;
use super::super::island_coord_group::IslandCoordGroup;
use super::super::island_coord_sep::IslandCoordSep;
use super::super::island_mini_grid::{IslandMiniGrid, IslandMiniGridProps};
use super::super::island_row_meta::IslandRowMeta;
use super::props::IslandSidebarProps;
use dioxus::prelude::*;

/// One finished card per collision island, with its selected state, key, click
/// handler, and content (mini grid + coordinates + collision count).
pub(super) fn cards(props: &IslandSidebarProps) -> Vec<CollisionCardProps> {
    let mut selected_island = props.selected_island;
    let selected_key = selected_island.read().clone();
    props
        .islands
        .iter()
        .map(|island| {
            let collision_key = island.key().to_owned();
            let is_selected = selected_key.as_deref() == Some(island.key());
            let collision_column = island.position_column();
            let collision_row = island.position_row();
            let collision_count = island.collision_count();
            let noun = if collision_count == 1 {
                "collision"
            } else {
                "collisions"
            };
            let column_text = format!("Column {collision_column}");
            let row_text = format!("Row {collision_row}");
            let count_text = format!("{collision_count} {noun}");
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_island.set(Some(key_for_click.clone()))
            });
            let grid = IslandMiniGridProps {
                collision_column,
                collision_row,
            };
            let children = rsx! {
                IslandMiniGrid { ..grid }
                IslandRowMeta {
                    IslandCoordGroup {
                        IslandCoord { text: column_text }
                        IslandCoordSep {}
                        IslandCoord { text: row_text }
                    }
                    IslandCollisionCount { text: count_text }
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
