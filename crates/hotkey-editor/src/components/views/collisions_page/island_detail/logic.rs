use super::components::island_conflict_card::IslandConflictCardProps;
use super::props::IslandDetailProps;
use crate::components::views::collisions_page::CarrierDialogData;
use dioxus::prelude::*;

/// The selected island's header coordinates and its per-unit conflict cards.
pub(super) struct IslandDetailModel {
    pub(super) position_column: u8,
    pub(super) position_row: u8,
    pub(super) column_label: String,
    pub(super) row_label: String,
    pub(super) count_text: String,
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
    let position_column = island.position_column();
    let position_row = island.position_row();
    let column_label = format!("Column {position_column}");
    let row_label = format!("Row {position_row}");
    let collision_count = island.collision_count();
    let noun = if collision_count == 1 {
        "collision"
    } else {
        "collisions"
    };
    let count_text = format!("{collision_count} {noun}");
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
        position_column,
        position_row,
        column_label,
        row_label,
        count_text,
        cards,
    })
}
