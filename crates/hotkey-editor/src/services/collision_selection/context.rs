use dioxus::prelude::*;

use crate::services::collision_selection::CollisionSelection;

pub(crate) fn use_collision_selection() -> CollisionSelection {
    use_context()
}

pub(crate) fn use_collision_selection_provider(
    initial_island: Option<String>,
    initial_hotkey_unit: Option<String>,
    initial_unit_position: Option<String>,
) -> CollisionSelection {
    let selected_island = use_signal::<Option<String>>(move || initial_island);
    let selected_hotkey_unit = use_signal::<Option<String>>(move || initial_hotkey_unit);
    let selected_unit_position = use_signal::<Option<String>>(move || initial_unit_position);
    let collision_selection = CollisionSelection::new(
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
    );
    use_context_provider(|| collision_selection);
    collision_selection
}
