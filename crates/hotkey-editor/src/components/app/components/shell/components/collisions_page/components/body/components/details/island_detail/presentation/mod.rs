use super::model::IslandDetailModel;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

/// Resolves the selected island, or `None` when nothing is selected.
pub(super) fn selected(
    props: &IslandDetailModel,
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
