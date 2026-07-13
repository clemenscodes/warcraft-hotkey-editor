use super::model::HelpFooterModel;
use crate::persistence::onboarding_persistence;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The footer host's shaped data: the dismiss handler for the button. Dismissing marks the
/// onboarding as seen so the guide stops auto-opening, then closes it via the shared signal.
pub(super) struct HelpFooterPresentation {
    pub(super) on_dismiss: EventHandler<MouseEvent>,
}

/// Reads the overlay context and builds the dismiss handler: it records the onboarding as
/// seen and flips the shared open signal closed.
pub(super) fn use_help_footer() -> HelpFooterPresentation {
    let overlay = use_overlay_state();
    let mut help_open = overlay.help_open();
    let on_dismiss = EventHandler::new(move |_event: MouseEvent| {
        onboarding_persistence::mark_seen();
        help_open.set(false);
    });
    HelpFooterPresentation { on_dismiss }
}

impl ddd::Presentation for HelpFooterPresentation {
    type Model = HelpFooterModel;
}
