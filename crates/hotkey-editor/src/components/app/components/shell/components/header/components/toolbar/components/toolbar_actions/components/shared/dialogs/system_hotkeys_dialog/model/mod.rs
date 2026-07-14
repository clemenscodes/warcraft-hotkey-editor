use super::view::SystemHotkeysDialogView;
use dioxus::prelude::*;

/// What the system-hotkeys dialog needs: the open value it drives and the change handler
/// mirroring the headless dialog's own close back to the trigger that owns the open signal.
/// The category tabs, editing section, and inventory drag are the dialog's own regions and
/// live in its provided UI state, so nothing else is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&SystemHotkeysDialogView> for SystemHotkeysDialogModel {
    fn from(view: &SystemHotkeysDialogView) -> Self {
        let SystemHotkeysDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for SystemHotkeysDialogModel {
    type View = SystemHotkeysDialogView;
}
