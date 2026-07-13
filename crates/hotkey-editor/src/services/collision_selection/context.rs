use dioxus::prelude::*;

use crate::services::collision_selection::CollisionSelection;

/// Access the [`CollisionSelection`] provided by the app shell. Call from a
/// component or hook body (it is a hook). The per-kind selection is shell-wide (it
/// outlives the page and feeds the URL sync), so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_collision_selection() -> CollisionSelection {
    use_context()
}

/// Create the three per-kind selection signals (each seeded from the entry the URL
/// carried for its kind), assemble the [`CollisionSelection`], provide it as context,
/// and hand it back. The shell calls this once on boot; every collisions component
/// reads the result through [`use_collision_selection`].
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
