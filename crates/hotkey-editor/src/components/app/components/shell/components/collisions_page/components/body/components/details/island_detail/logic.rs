use super::components::island_conflict_card::IslandConflictCardProps;
use super::props::IslandDetailProps;
use crate::components::app::components::shell::components::collisions_page::logic::CarrierDialogData;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The selected island's header coordinate and its per-unit conflict cards.
pub(super) struct IslandDetailModel {
    pub(super) coordinate: GridCoordinate,
    pub(super) count: usize,
    pub(super) cards: Vec<IslandConflictCardProps>,
}

/// Resolves the selected island and shapes its header and conflict cards, or `None`
/// when nothing is selected (the pane shows its empty prompt).
pub(super) fn selected(
    props: &IslandDetailProps,
    carrier_dialog: Signal<Option<CarrierDialogData>>,
) -> Option<IslandDetailModel> {
    let selected_key = props.selected_island.read().clone();
    let key = selected_key?;
    let island = props
        .islands
        .iter()
        .find(|island| island.key() == key)?
        .clone();
    let coordinate = island.coordinate();
    let collision_count = island.collision_count();
    let cards = island
        .conflicts()
        .iter()
        .map(|conflict| IslandConflictCardProps {
            conflict: conflict.clone(),
            carrier_dialog,
            view_navigation: props.view_navigation,
        })
        .collect();
    Some(IslandDetailModel {
        coordinate,
        count: collision_count,
        cards,
    })
}
