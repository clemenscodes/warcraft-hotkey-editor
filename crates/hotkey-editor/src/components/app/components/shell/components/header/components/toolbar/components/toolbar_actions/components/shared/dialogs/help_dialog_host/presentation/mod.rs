use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The help dialog's shaped wiring: whether the guide is open, and the change handler that
/// mirrors the headless dialog's own close (escape, outside click) back to the shared
/// signal. It also locks body scroll while the guide is open — the dialog is mounted here
/// (the burger only flips the shared signal), so the lock lives with this always-mounted
/// host.
pub(super) struct HelpDialogHostModel {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the overlay context and shapes the help dialog wiring: the shared open signal the
/// help button flips and first-visit onboarding sets, plus the change handler mirroring the
/// headless dialog's close back to that signal.
pub(super) fn use_help_dialog_host() -> HelpDialogHostModel {
    let overlay = use_overlay_state();
    let help_open = overlay.help_open();
    use_body_scroll_lock(help_open);
    let open = *help_open.read();
    let mut change_open = help_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    HelpDialogHostModel {
        open,
        on_open_change,
    }
}
