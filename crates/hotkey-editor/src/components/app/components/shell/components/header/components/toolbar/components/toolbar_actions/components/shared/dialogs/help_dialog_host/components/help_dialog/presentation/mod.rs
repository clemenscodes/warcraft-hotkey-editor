use super::data::{HELP_CONTENT, HelpContent};
use super::model::HelpDialogModel;
use crate::persistence::onboarding_persistence;
use dioxus::prelude::*;

/// The help dialog's own shell, shaped directly from its props: the open value driving
/// the backdrop, the change handler that writes the open signal, the header title and
/// close handler, the guide content, and the dismiss handler. The panel is built from
/// these plain values at the render site, not carried as its props. Every dialog owns
/// its shell now — there is no base.
pub(super) struct HelpDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) content: HelpContent,
    pub(super) on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDialogModel> for HelpDialogShell {
    fn from(props: &HelpDialogModel) -> Self {
        let mut open_signal = props.help_open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = props.help_open;
        let title = String::from("How to use this editor");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let content = HELP_CONTENT;
        let mut dismiss_signal = props.help_open;
        let on_dismiss = EventHandler::new(move |_event: MouseEvent| {
            onboarding_persistence::mark_seen();
            dismiss_signal.set(false);
        });
        Self {
            open,
            on_open_change,
            title,
            on_close,
            content,
            on_dismiss,
        }
    }
}

impl ddd::Presentation for HelpDialogShell {
    type Model = HelpDialogModel;
}
