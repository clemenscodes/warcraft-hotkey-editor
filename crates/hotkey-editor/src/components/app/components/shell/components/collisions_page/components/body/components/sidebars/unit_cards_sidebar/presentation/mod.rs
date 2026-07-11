use super::model::UnitCardsSidebarModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::{CollisionCardContent, CollisionCardData};
use crate::components::app::components::shell::components::collisions_page::presentation::{
    HotkeyConflictView, UnitPositionConflictView,
};
use crate::services::collision_selection::CollisionSelection;
use dioxus::prelude::*;

/// Maps a collision conflict kind to the collision-selection field that names the
/// selected unit for that kind, so the shared generic sidebar reads its own selection
/// from context instead of receiving it drilled as a prop — mirroring how
/// `IslandSidebar` and the detail panes read their selection.
pub(super) trait SelectedCollisionUnit {
    fn selected_unit(collision_selection: CollisionSelection) -> Signal<Option<String>>;
}

impl SelectedCollisionUnit for HotkeyConflictView {
    fn selected_unit(collision_selection: CollisionSelection) -> Signal<Option<String>> {
        collision_selection.selected_hotkey_unit()
    }
}

impl SelectedCollisionUnit for UnitPositionConflictView {
    fn selected_unit(collision_selection: CollisionSelection) -> Signal<Option<String>> {
        collision_selection.selected_unit_position()
    }
}

/// One card's data per clashing unit: its portrait, name, id, and clash count. The
/// selected unit arrives already read from context by the component.
pub(super) fn cards<Conflict: Clone + PartialEq + 'static>(
    props: &UnitCardsSidebarModel<Conflict>,
    mut selected_unit: Signal<Option<String>>,
) -> Vec<CollisionCardData> {
    let selected_key = selected_unit.read().clone();
    props
        .units
        .iter()
        .map(|unit_view| {
            let is_selected = selected_key.as_deref() == Some(unit_view.key());
            let unit = unit_view.unit();
            let icon_url = unit.icon_url().map(str::to_owned);
            let name = unit.name().to_owned();
            let unit_id = unit.unit_id();
            let collision_count = unit_view.collision_count();
            let key_for_click = unit_view.key().to_owned();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_unit.set(Some(key_for_click.clone()))
            });
            let content = CollisionCardContent::Unit {
                icon_url,
                name,
                unit_id,
            };
            CollisionCardData {
                is_selected,
                onclick,
                count: collision_count,
                content,
            }
        })
        .collect()
}
