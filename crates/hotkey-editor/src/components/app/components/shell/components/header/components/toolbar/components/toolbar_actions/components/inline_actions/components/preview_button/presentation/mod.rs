use super::data::{HIDE_LABEL, LABEL};
use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use dioxus::prelude::*;

pub(super) struct PreviewButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_pressed: Option<bool>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_preview_button() -> PreviewButtonPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_label = if open { HIDE_LABEL } else { LABEL };
    let aria_pressed = Some(open);
    PreviewButtonPresentation {
        icon: ICON_PREVIEW,
        aria_label,
        aria_pressed,
        open,
        onclick,
        on_open_change,
    }
}
