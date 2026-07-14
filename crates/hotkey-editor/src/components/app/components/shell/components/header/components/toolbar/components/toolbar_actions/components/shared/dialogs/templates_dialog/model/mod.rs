use super::view::TemplatesDialogView;
use dioxus::prelude::*;

/// What the templates dialog needs: the open value it drives and the change handler mirroring
/// the headless dialog's own close back to the trigger that owns the open signal. The
/// gallery body is the dialog's own region, so nothing else is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&TemplatesDialogView> for TemplatesDialogModel {
    fn from(view: &TemplatesDialogView) -> Self {
        let TemplatesDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for TemplatesDialogModel {
    type View = TemplatesDialogView;
}
