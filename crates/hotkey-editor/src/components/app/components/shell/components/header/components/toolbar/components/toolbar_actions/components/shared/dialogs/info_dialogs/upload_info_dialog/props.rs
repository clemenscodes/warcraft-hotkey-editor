use dioxus::prelude::*;

/// What the import dialog needs: the open signal it drives. The picker itself is
/// a web API service the action row triggers.
#[derive(Props, Clone, PartialEq)]
pub struct UploadInfoDialogProps {
    pub open: Signal<bool>,
}
