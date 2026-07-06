use dioxus::prelude::*;

use crate::services::focus::coordinator::FocusCoordinator;

/// Access the [`FocusCoordinator`] provided at the app shell root. Call from a
/// component or hook body (it is a hook). Keyboard focus hand-off is a global,
/// crate-wide concern owned by the focus service, so this accessor lives beside the
/// type in `services/focus/`, not colocated with any component.
pub(crate) fn use_focus_coordinator() -> FocusCoordinator {
    use_context()
}
