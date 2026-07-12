use super::view::UploadInfoDialogView;
use dioxus::prelude::*;

/// What the import dialog needs: the open value it drives and the change handler
/// mirroring the headless dialog's own close. The picker itself is a web API
/// service the action row triggers.
#[derive(Props, Clone, PartialEq)]
pub struct UploadInfoDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&UploadInfoDialogView> for UploadInfoDialogModel {
    fn from(view: &UploadInfoDialogView) -> Self {
        let UploadInfoDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for UploadInfoDialogModel {
    type View = UploadInfoDialogView;
}
