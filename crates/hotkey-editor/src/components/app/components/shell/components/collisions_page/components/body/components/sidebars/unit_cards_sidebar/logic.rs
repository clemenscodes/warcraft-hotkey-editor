use super::props::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::{CollisionCardContent, CollisionCardProps};
use dioxus::prelude::*;

/// One card's data per clashing unit: its portrait, name, id, and clash count.
pub(super) fn cards<Conflict: Clone + PartialEq + 'static>(
    props: &UnitCardsSidebarProps<Conflict>,
) -> Vec<CollisionCardProps> {
    let mut selected_unit = props.selected_unit;
    let selected_key = selected_unit.read().clone();
    props
        .units
        .iter()
        .map(|unit_view| {
            let collision_key = unit_view.key().to_owned();
            let is_selected = selected_key.as_deref() == Some(unit_view.key());
            let unit = unit_view.unit();
            let icon_url = unit.icon_url().map(str::to_owned);
            let name = unit.name().to_owned();
            let unit_id = unit.unit_id().to_owned();
            let collision_count = unit_view.collision_count();
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_unit.set(Some(key_for_click.clone()))
            });
            let content = CollisionCardContent::Unit {
                icon_url,
                name,
                unit_id,
            };
            CollisionCardProps {
                is_selected,
                collision_key,
                onclick,
                count: collision_count,
                content,
            }
        })
        .collect()
}
