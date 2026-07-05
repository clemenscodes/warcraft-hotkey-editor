use dioxus::prelude::*;

use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::navigation::view_navigation::ViewNavigationContext;

/// Access the [`ViewNavigationContext`] provided at the app root. Call from a
/// component or hook body (it is a hook). Navigation is a global, crate-wide
/// concept — no single component owns it — so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_view_navigation() -> ViewNavigationContext {
    use_context()
}

/// Access the shell's URL guard — the [`NavSnapshot`] mirroring what the address bar
/// currently shows. A page peeks-and-sets it when it reconciles the route it was
/// mounted for, so the shell's push effect can tell an echo of a browser
/// back/forward (skip) from a genuine state change (navigate). Provided by the app
/// shell, so this accessor lives beside the type in `services/`.
pub(crate) fn use_synced_route() -> Signal<NavSnapshot> {
    use_context()
}
