use super::model::InfoDialogConfig;
use dioxus::prelude::*;

/// The info dialog's own shell, shaped from its config: the open value driving the
/// dialog, the change handler that writes the open signal (mirroring the headless
/// dialog's own close), the header title, and the flat copy and action data the body
/// region forwards to its instruction block and action row. The title/close header is
/// `WarcraftDialog`'s own chrome, so no close handler is shaped here.
pub(super) struct InfoDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: &'static str,
    pub(super) intro: &'static str,
    pub(super) warning: Option<&'static str>,
    pub(super) primary_label: &'static str,
    pub(super) on_primary: EventHandler<MouseEvent>,
    pub(super) on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoDialogConfig> for InfoDialogShell {
    fn from(props: &InfoDialogConfig) -> Self {
        let open = props.open;
        let on_open_change = props.on_open_change;
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
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}
