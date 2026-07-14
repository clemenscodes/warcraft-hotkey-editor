use super::model::IslandDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

pub(super) const EMPTY_PROMPT: &str = "Select a collision to inspect.";

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
