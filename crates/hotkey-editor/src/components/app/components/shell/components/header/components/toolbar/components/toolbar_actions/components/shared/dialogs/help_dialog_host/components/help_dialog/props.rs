use super::view::HelpDialogView;
use dioxus::prelude::*;

/// The help dialog's only input: the shared open signal it drives. The dialog
/// auto-opens on first visit and reopens from the toolbar help button.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogProps {
    pub help_open: Signal<bool>,
}

impl From<&HelpDialogView> for HelpDialogProps {
    fn from(view: &HelpDialogView) -> Self {
        let HelpDialogView { help_open } = view.clone();
        Self { help_open }
    }
}

impl ddd::Props for HelpDialogProps {
    type View = HelpDialogView;
}
