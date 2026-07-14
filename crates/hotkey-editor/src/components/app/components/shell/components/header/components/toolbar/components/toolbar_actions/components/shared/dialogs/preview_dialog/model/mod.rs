use super::view::PreviewDialogView;
use dioxus::prelude::*;

/// What the preview dialog needs: the open value it drives and the change handler mirroring
/// the headless dialog's own close back to the trigger that owns the open signal. The
/// serialized-preview body is the dialog's own region, so nothing else is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&PreviewDialogView> for PreviewDialogModel {
    fn from(view: &PreviewDialogView) -> Self {
        let PreviewDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for PreviewDialogModel {
    type View = PreviewDialogView;
}
