use super::view::UploadInfoDialogView;
use dioxus::prelude::*;

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
