use dioxus::prelude::*;

use crate::services::collision_selection::CollisionSelection;

/// Access the [`CollisionSelection`] provided by the app shell. Call from a
/// component or hook body (it is a hook). The per-kind selection is shell-wide (it
/// outlives the page and feeds the URL sync), so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_collision_selection() -> CollisionSelection {
    use_context()
}
