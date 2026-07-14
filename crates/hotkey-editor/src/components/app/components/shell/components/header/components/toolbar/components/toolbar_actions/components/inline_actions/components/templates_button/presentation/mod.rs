use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_TEMPLATES;
use dioxus::prelude::*;

/// The templates button's shaped data: the fixed icon and label, its aria state, whether the
/// templates browser is open, the toggle handler, and the change handler the mounted dialog
/// mirrors its own close through. The open signal is local and owned here — the button that
/// opens the dialog owns it, so the dialog travels with it.
pub(super) struct TemplatesButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Owns the templates browser's local open signal and shapes the button's data: the toggle
/// handler that opens or closes the browser, and the change handler the mounted dialog mirrors
/// its own close through.
pub(super) fn use_templates_button() -> TemplatesButtonPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_haspopup = Some("dialog");
    let aria_expanded = Some(open);
    TemplatesButtonPresentation {
        icon: ICON_TEMPLATES,
        aria_label: ARIA_LABEL,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    }
}
