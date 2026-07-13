use super::model::UnitCardsSidebarModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::{CollisionCardContent, CollisionCardData};
use crate::components::app::components::shell::components::collisions_page::presentation::{
    HotkeyConflictView, UnitPositionConflictView,
};
use crate::services::collision_selection::CollisionSelection;
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::marker::PhantomData;

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

/// The unit sidebar's render-ready cards: one card per clashing unit. Generic over the
/// conflict shape so the hotkey and unit-position kinds share one builder. The body only
/// places these; all shaping happens in the builder below.
pub(super) struct UnitCardsSidebarPresentation<Conflict: Clone + PartialEq + 'static> {
    pub(super) cards: Vec<CollisionCardData>,
    conflict: PhantomData<Conflict>,
}

impl<Conflict: Clone + PartialEq + 'static> ddd::Presentation
    for UnitCardsSidebarPresentation<Conflict>
{
    type Model = UnitCardsSidebarModel<Conflict>;
}

/// Reads the selected unit (the conflict kind names which field) and the navigation
/// context, then shapes one card per clashing unit: its portrait, name, id, and clash
/// count. The click routes through navigation, which replaces the collisions route's
/// `?entry=` with the picked unit for the active kind.
pub(super) fn use_unit_cards_sidebar_presentation<
    Conflict: Clone + PartialEq + SelectedCollisionUnit + 'static,
>(
    props: &UnitCardsSidebarModel<Conflict>,
) -> UnitCardsSidebarPresentation<Conflict> {
    let collision_selection = use_collision_selection();
    let selected_unit = Conflict::selected_unit(collision_selection);
    let view_navigation = use_view_navigation();
    let selected_key = selected_unit.read().clone();
    let cards = props
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
                view_navigation.select_collision_entry(key_for_click.clone())
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
        .collect();
    let conflict = PhantomData;
    UnitCardsSidebarPresentation { cards, conflict }
}
