use super::view::UploadInfoDialogView;
use dioxus::prelude::*;

/// What the import dialog needs: the open signal it drives. The picker itself is
/// a web API service the action row triggers.
#[derive(Props, Clone, PartialEq)]
pub struct UploadInfoDialogProps {
    pub open: Signal<bool>,
}

impl From<&UploadInfoDialogView> for UploadInfoDialogProps {
    fn from(view: &UploadInfoDialogView) -> Self {
        let UploadInfoDialogView { open } = view.clone();
        Self { open }
    }
}

impl ddd::Props for UploadInfoDialogProps {
    type View = UploadInfoDialogView;
}
