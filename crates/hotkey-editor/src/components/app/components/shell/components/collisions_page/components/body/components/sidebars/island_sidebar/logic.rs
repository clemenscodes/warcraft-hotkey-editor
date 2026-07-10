use super::props::IslandSidebarProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::{CollisionCardContent, CollisionCardData};
use dioxus::prelude::*;

/// One card's data per collision island: its selected state, key, click handler,
/// highlighted coordinate, and collision-count line. The selection is read from context
/// by the caller and passed in.
pub(super) fn cards(
    props: &IslandSidebarProps,
    mut selected_island: Signal<Option<String>>,
) -> Vec<CollisionCardData> {
    let selected_key = selected_island.read().clone();
    props
        .islands
        .iter()
        .map(|island| {
            let is_selected = selected_key.as_deref() == Some(island.key());
            let coordinate = island.coordinate();
            let collision_count = island.collision_count();
            let key_for_click = island.key().to_owned();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_island.set(Some(key_for_click.clone()))
            });
            let content = CollisionCardContent::Island { coordinate };
            CollisionCardData {
                is_selected,
                onclick,
                count: collision_count,
                content,
            }
        })
        .collect()
}
