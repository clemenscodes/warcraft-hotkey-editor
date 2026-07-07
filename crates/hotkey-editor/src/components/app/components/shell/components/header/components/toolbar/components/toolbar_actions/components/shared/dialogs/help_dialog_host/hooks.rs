use super::super::help_dialog::HelpDialogProps;
use crate::services::overlay_state::context::use_overlay_state;

/// Shapes the help dialog's only prop from context: the shared open signal the help
/// button flips and the first-visit onboarding sets.
pub(super) fn use_help_dialog_host() -> HelpDialogProps {
    let overlay = use_overlay_state();
    let help_open = overlay.help_open();
    HelpDialogProps { help_open }
}
