use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_COG;
use dioxus::prelude::*;

pub(super) struct SystemHotkeysButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_system_hotkeys_button() -> SystemHotkeysButtonPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_haspopup = Some("dialog");
    let aria_expanded = Some(open);
    SystemHotkeysButtonPresentation {
        icon: ICON_COG,
        aria_label: ARIA_LABEL,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    }
}
