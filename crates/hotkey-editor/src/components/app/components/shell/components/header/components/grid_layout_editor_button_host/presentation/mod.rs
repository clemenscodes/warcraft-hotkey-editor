use crate::components::app::components::shell::components::shared::dialog_toggle::use_dialog_toggle;
use dioxus::prelude::*;

pub(super) struct GridLayoutEditorButtonPresentation {
    pub is_open: bool,
    pub onpointerdown: EventHandler<PointerEvent>,
    pub onclick: EventHandler<MouseEvent>,
    pub on_open_change: Callback<bool>,
}

pub(super) fn use_grid_layout_editor_button() -> GridLayoutEditorButtonPresentation {
    let open_signal = use_signal::<bool>(|| false);
    let is_open = open_signal();
    // The host remembers on pointerdown whether the dialog was open, because a
    // press outside the open dialog dismisses it before this click arrives.
    let toggle = use_dialog_toggle(open_signal);
    let onpointerdown = toggle.onpointerdown();
    let onclick = toggle.onclick();
    let on_open_change = toggle.on_open_change();
    GridLayoutEditorButtonPresentation {
        is_open,
        onpointerdown,
        onclick,
        on_open_change,
    }
}
