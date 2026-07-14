use dioxus::prelude::*;

use crate::persistence::onboarding_persistence;
use crate::services::overlay_state::OverlayState;

/// Access the [`OverlayState`] provided at the app root. Call from a component or
/// hook body (it is a hook). Overlay visibility is a global, crate-wide concept —
/// no single component owns it — so this accessor lives beside the type in
/// `services/`, not colocated with any component.
pub(crate) fn use_overlay_state() -> OverlayState {
    use_context()
}

/// Create the five overlay-visibility signals, assemble the [`OverlayState`], provide
/// it as context, and hand it back. The help dialog opens on a first visit (until the
/// onboarding has been seen); every other overlay starts closed. The shell calls this
/// once on boot; the header and burger drawer read the result through
/// [`use_overlay_state`].
pub(crate) fn use_overlay_state_provider() -> OverlayState {
    let preview_open = use_signal::<bool>(|| false);
    let system_hotkeys_open = use_signal::<bool>(|| false);
    let help_open = use_signal::<bool>(|| !onboarding_persistence::has_been_seen());
    let layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);
    let upload_info_open = use_signal::<bool>(|| false);
    let download_info_open = use_signal::<bool>(|| false);
    let overlay_state = OverlayState::new(
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
        upload_info_open,
        download_info_open,
    );
    use_context_provider(|| overlay_state);
    overlay_state
}
