use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The help dialog's shaped wiring: whether the guide is open, and the change handler that
/// mirrors the headless dialog's own close (escape, outside click) back to the shared
/// signal. Body-scroll lock is owned once by `WarcraftDialog`, so this host only flips the
/// shared signal.
pub(super) struct HelpDialogModel {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the overlay context and shapes the help dialog wiring: the shared open signal the
/// help button flips and first-visit onboarding sets, plus the change handler mirroring the
/// headless dialog's close back to that signal.
pub(super) fn use_help_dialog() -> HelpDialogModel {
    let overlay = use_overlay_state();
    let help_open = overlay.help_open();
    let open = *help_open.read();
    let mut change_open = help_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    HelpDialogModel {
        open,
        on_open_change,
    }
}
