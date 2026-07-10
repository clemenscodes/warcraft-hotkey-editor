use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::HelpDialogProps;
use crate::persistence::onboarding_persistence;
use dioxus::prelude::*;

/// The dismiss button's only input: the click handler. Built from the dialog
/// props so it both marks the onboarding seen and closes the dialog.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDismissProps {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDialogProps> for HelpDismissProps {
    fn from(props: &HelpDialogProps) -> Self {
        let mut help_open = props.help_open;
        let on_dismiss = EventHandler::new(move |_event: MouseEvent| {
            onboarding_persistence::mark_seen();
            help_open.set(false);
        });
        Self { on_dismiss }
    }
}
