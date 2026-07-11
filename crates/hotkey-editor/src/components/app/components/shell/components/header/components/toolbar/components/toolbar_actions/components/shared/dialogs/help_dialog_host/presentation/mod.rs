use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// Shapes the help dialog's only input from context: the shared open signal the help
/// button flips and the first-visit onboarding sets.
pub(super) fn use_help_dialog_host() -> Signal<bool> {
    let overlay = use_overlay_state();
    overlay.help_open()
}
