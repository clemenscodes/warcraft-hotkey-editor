use super::model::IslandSidebarModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::{CollisionCardContent, CollisionCardData};
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The island sidebar's render-ready cards: one card per collision island. The body only
/// places these; all shaping happens in the builder below.
pub(super) struct IslandSidebarPresentation {
    pub(super) cards: Vec<CollisionCardData>,
}

impl ddd::Presentation for IslandSidebarPresentation {
    type Model = IslandSidebarModel;
}

/// Reads the selected island and the navigation context, then shapes one card per
/// collision island: its selected state, key, click handler, highlighted coordinate, and
/// collision-count line. The click routes through navigation, which replaces the
/// collisions route's `?entry=` with the picked island.
pub(super) fn use_island_sidebar_presentation(
    props: &IslandSidebarModel,
) -> IslandSidebarPresentation {
    let selected_island = use_collision_selection().selected_island();
    let view_navigation = use_view_navigation();
    let selected_key = selected_island.read().clone();
    let cards = props
        .islands
        .iter()
        .map(|island| {
            let is_selected = selected_key.as_deref() == Some(island.key());
            let coordinate = island.coordinate();
            let collision_count = island.collision_count();
            let key_for_click = island.key().to_owned();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                view_navigation.select_collision_entry(key_for_click.clone())
            });
            let content = CollisionCardContent::Island { coordinate };
            CollisionCardData {
                is_selected,
                onclick,
                count: collision_count,
                content,
            }
        })
        .collect();
    IslandSidebarPresentation { cards }
}
