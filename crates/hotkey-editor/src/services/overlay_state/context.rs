use dioxus::prelude::*;

use crate::services::overlay_state::OverlayState;

/// Access the [`OverlayState`] provided at the app root. Call from a component or
/// hook body (it is a hook). Overlay visibility is a global, crate-wide concept —
/// no single component owns it — so this accessor lives beside the type in
/// `services/`, not colocated with any component.
pub(crate) fn use_overlay_state() -> OverlayState {
    use_context()
}
