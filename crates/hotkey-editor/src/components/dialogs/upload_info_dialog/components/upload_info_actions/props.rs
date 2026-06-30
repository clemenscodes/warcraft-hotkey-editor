use dioxus::prelude::*;

use crate::components::dialogs::upload_info_dialog::UploadInfoDialogProps;
use crate::services::files::upload::UploadPicker;

/// The two action handlers, built from the dialog props. Cancel closes the
/// dialog; choose-file closes it and opens the native file picker.
#[derive(Props, Clone, PartialEq)]
pub struct UploadInfoActionsProps {
    pub on_cancel: EventHandler<MouseEvent>,
    pub on_choose_file: EventHandler<MouseEvent>,
}

impl From<&UploadInfoDialogProps> for UploadInfoActionsProps {
    fn from(props: &UploadInfoDialogProps) -> Self {
        let mut open = props.open;
        let on_cancel = EventHandler::new(move |_event: MouseEvent| open.set(false));
        let on_choose_file = EventHandler::new(move |_event: MouseEvent| {
            open.set(false);
            UploadPicker::trigger();
        });
        Self {
            on_cancel,
            on_choose_file,
        }
    }
}
