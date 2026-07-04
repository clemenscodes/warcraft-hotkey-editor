use super::components::island_card::IslandCardProps;
use super::props::IslandSidebarProps;
use dioxus::prelude::*;

/// One card's data per collision island: its selected state, key, click handler,
/// highlighted coordinate, and collision-count line.
pub(super) fn cards(props: &IslandSidebarProps) -> Vec<IslandCardProps> {
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
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_island.set(Some(key_for_click.clone()))
            });
            IslandCardProps {
                is_selected,
                collision_key,
                onclick,
                coordinate,
                count: collision_count,
            }
        })
        .collect()
}
