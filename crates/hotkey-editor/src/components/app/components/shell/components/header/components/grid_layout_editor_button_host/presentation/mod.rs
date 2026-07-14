use dioxus::prelude::*;

/// Shaped grid-layout button data: whether the layout editor is open (for aria), the toggle
/// handler that opens it, and the change handler the mounted dialog mirrors its own close
/// through. The open signal is local and owned here — this button opens the dialog, so it owns
/// the signal and the dialog travels with it.
pub(super) struct GridLayoutEditorButtonPresentation {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub on_open_change: Callback<bool>,
}

/// Owns the grid-layout editor's local open signal and shapes the button's data: the toggle
/// handler that opens or closes the editor, and the change handler the mounted dialog mirrors
/// its own close through.
pub(super) fn use_grid_layout_editor_button() -> GridLayoutEditorButtonPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let is_open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |value: bool| open_signal.set(value));
    GridLayoutEditorButtonPresentation {
        is_open,
        onclick,
        on_open_change,
    }
}
