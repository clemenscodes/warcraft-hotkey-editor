use super::model::IslandDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

/// The prompt shown in the island detail pane before a collision is selected.
pub(super) const EMPTY_PROMPT: &str = "Select a collision to inspect.";

/// Resolves the selected island, or `None` when nothing is selected.
pub(super) fn selected(
    props: &IslandDetailBodyModel,
    selected_island: Signal<Option<String>>,
) -> Option<IslandView> {
    let selected_key = selected_island.read().clone();
    let key = selected_key?;
    let island = props
        .islands
        .iter()
        .find(|island| island.key() == key)?
        .clone();
    Some(island)
}
