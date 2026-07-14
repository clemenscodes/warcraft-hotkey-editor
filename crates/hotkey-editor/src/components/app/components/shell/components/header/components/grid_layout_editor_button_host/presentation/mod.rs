use dioxus::prelude::*;

pub(super) struct GridLayoutEditorButtonPresentation {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub on_open_change: Callback<bool>,
}

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
