use dioxus::prelude::*;

/// The help dialog's only input: the shared open signal it drives. The dialog
/// auto-opens on first visit and reopens from the toolbar help button.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogProps {
    pub help_open: Signal<bool>,
}
