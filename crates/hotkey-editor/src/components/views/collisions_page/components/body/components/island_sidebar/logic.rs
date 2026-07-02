use super::super::collision_card::CollisionCardProps;
use super::super::collision_count::CollisionCount;
use super::super::coordinate::Coordinate;
use super::super::coordinate_group::CoordinateGroup;
use super::super::coordinate_separator::CoordinateSeparator;
use super::super::mini_grid::{MiniGrid, MiniGridProps};
use super::super::row_meta::RowMeta;
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
            let grid = MiniGridProps {
                collision_column,
                collision_row,
            };
            let children = rsx! {
                MiniGrid { ..grid }
                RowMeta {
                    CoordinateGroup {
                        Coordinate { text: column_text }
                        CoordinateSeparator {}
                        Coordinate { text: row_text }
                    }
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
