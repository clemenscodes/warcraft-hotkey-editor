use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_HELP;
use crate::persistence::onboarding_persistence;
use dioxus::prelude::*;

/// The help button's shaped data: the fixed icon and accessible label, whether its guide is
/// open (for aria), the toggle handler, and the change handler the mounted dialog mirrors its
/// own close back through. The open signal is local and owned here — the button that opens the
/// dialog owns it, so the dialog travels with the button.
pub(super) struct HelpButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Owns the help guide's local open signal — initialized open on a first visit so the guide
/// onboards, closed thereafter — and shapes the button's data: the toggle handler that opens
/// or closes it, and the change handler the mounted dialog mirrors its own close through.
pub(super) fn use_help_button() -> HelpButtonPresentation {
    let mut open_signal = use_signal::<bool>(|| !onboarding_persistence::has_been_seen());
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_haspopup = Some("dialog");
    let aria_expanded = Some(open);
    HelpButtonPresentation {
        icon: ICON_HELP,
        aria_label: ARIA_LABEL,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    }
}
