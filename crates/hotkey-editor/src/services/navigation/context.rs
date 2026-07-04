use dioxus::prelude::*;

use crate::services::navigation::view_navigation::ViewNavigationContext;

/// Access the [`ViewNavigationContext`] provided at the app root. Call from a
/// component or hook body (it is a hook). Navigation is a global, crate-wide
/// concept — no single component owns it — so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_view_navigation() -> ViewNavigationContext {
    use_context::<ViewNavigationContext>()
}
