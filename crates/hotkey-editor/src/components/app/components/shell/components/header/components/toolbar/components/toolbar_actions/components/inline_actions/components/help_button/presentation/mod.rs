use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::dialog_toggle::use_dialog_toggle;
use crate::components::app::components::shell::components::shared::icons::ICON_HELP;
use crate::persistence::onboarding_persistence::OnboardingPersistence;
use crate::services::viewport::use_is_pager_viewport;
use dioxus::prelude::*;

pub(super) struct HelpButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) open: bool,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_help_button() -> HelpButtonPresentation {
    // The guide opens on its own on a first visit, here only where this button
    // is shown; below the desktop band the burger menu's onboarding opens it.
    let pager_viewport = use_is_pager_viewport();
    let open_signal =
        use_signal::<bool>(move || !pager_viewport && !OnboardingPersistence::has_been_seen());
    let open = open_signal();
    // The host remembers on pointerdown whether the dialog was open, because a
    // press outside the open dialog dismisses it before this click arrives.
    let toggle = use_dialog_toggle(open_signal);
    let onpointerdown = toggle.onpointerdown();
    let onclick = toggle.onclick();
    let on_open_change = toggle.on_open_change();
    let aria_haspopup = Some("dialog");
    let aria_expanded = Some(open);
    HelpButtonPresentation {
        icon: ICON_HELP,
        aria_label: ARIA_LABEL,
        aria_haspopup,
        aria_expanded,
        open,
        onpointerdown,
        onclick,
        on_open_change,
    }
}
