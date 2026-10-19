use super::data::{HIDE_LABEL, LABEL};
use crate::components::app::components::shell::components::shared::dialog_toggle::use_dialog_toggle;
use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use dioxus::prelude::*;

pub(super) struct PreviewButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_pressed: Option<bool>,
    pub(super) open: bool,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_preview_button() -> PreviewButtonPresentation {
    let open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    // The host remembers on pointerdown whether the dialog was open, because a
    // press outside the open dialog dismisses it before this click arrives.
    let toggle = use_dialog_toggle(open_signal);
    let onpointerdown = toggle.onpointerdown();
    let onclick = toggle.onclick();
    let on_open_change = toggle.on_open_change();
    let aria_label = if open { HIDE_LABEL } else { LABEL };
    let aria_pressed = Some(open);
    PreviewButtonPresentation {
        icon: ICON_PREVIEW,
        aria_label,
        aria_pressed,
        open,
        onpointerdown,
        onclick,
        on_open_change,
    }
}
