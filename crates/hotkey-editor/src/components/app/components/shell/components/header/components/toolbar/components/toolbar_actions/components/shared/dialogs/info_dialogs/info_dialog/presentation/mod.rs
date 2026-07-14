use super::model::InfoDialogConfig;
use dioxus::prelude::*;

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
