use super::model::InfoDialogConfig;
use dioxus::prelude::*;

/// The info dialog's own shell, shaped from its config: the open value driving the
/// backdrop, the change handler that writes the open signal, the close handler for
/// the header, and the flat copy and action data the panel forwards to its header
/// and scroll-region body. Every dialog owns its shell now — there is no base.
pub(super) struct InfoDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: &'static str,
    pub(super) on_close: EventHandler<()>,
    pub(super) intro: &'static str,
    pub(super) warning: Option<&'static str>,
    pub(super) primary_label: &'static str,
    pub(super) on_primary: EventHandler<MouseEvent>,
    pub(super) on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoDialogConfig> for InfoDialogShell {
    fn from(props: &InfoDialogConfig) -> Self {
        let mut open_signal = props.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = props.open;
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let title = props.title;
        let intro = props.intro;
        let warning = props.warning;
        let primary_label = props.primary_label;
        let on_primary = props.on_primary;
        let on_cancel = props.on_cancel;
        Self {
            open,
            on_open_change,
            title,
            on_close,
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}
