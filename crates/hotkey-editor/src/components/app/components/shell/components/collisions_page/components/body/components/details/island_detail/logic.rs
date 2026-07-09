use super::components::island_conflict_grid::components::island_conflict_card::IslandConflictCardProps;
use super::props::IslandDetailProps;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The selected island's header coordinate and its per-unit conflict cards, shaped for
/// the detail pane to render. Each conflict ability owns and opens its own carriers
/// dialog, so this data knows nothing about it.
#[derive(Clone, PartialEq)]
pub(super) struct IslandDetailData {
    pub(super) coordinate: GridCoordinate,
    pub(super) count: usize,
    pub(super) cards: Vec<IslandConflictCardProps>,
}

/// Resolves the selected island and shapes its header coordinate and conflict cards,
/// or `None` when nothing is selected.
pub(super) fn selected(props: &IslandDetailProps) -> Option<IslandDetailData> {
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
        })
        .collect();
    let data = IslandDetailData {
        coordinate,
        count: collision_count,
        cards,
    };
    Some(data)
}
