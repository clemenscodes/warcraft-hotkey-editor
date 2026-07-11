use super::view::UploadInfoDialogView;
use dioxus::prelude::*;

/// What the import dialog needs: the open signal it drives. The picker itself is
/// a web API service the action row triggers.
#[derive(Props, Clone, PartialEq)]
pub struct UploadInfoDialogModel {
    pub open: Signal<bool>,
}

impl From<&UploadInfoDialogView> for UploadInfoDialogModel {
    fn from(view: &UploadInfoDialogView) -> Self {
        let UploadInfoDialogView { open } = view.clone();
        Self { open }
    }
}

impl ddd::Model for UploadInfoDialogModel {
    type View = UploadInfoDialogView;
}
