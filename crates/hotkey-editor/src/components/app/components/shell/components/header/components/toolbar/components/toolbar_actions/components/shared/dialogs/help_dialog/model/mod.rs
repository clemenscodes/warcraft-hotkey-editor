use super::view::HelpDialogView;
use dioxus::prelude::*;

/// What the help dialog needs: the open value it drives and the change handler mirroring
/// the headless dialog's own close back to the trigger that owns the open signal. The
/// guide body and dismiss footer are the dialog's own regions, so nothing else is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&HelpDialogView> for HelpDialogModel {
    fn from(view: &HelpDialogView) -> Self {
        let HelpDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for HelpDialogModel {
    type View = HelpDialogView;
}
