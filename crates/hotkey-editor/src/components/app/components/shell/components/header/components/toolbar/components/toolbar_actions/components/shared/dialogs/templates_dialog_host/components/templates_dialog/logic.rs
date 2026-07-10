use super::components::templates_dialog_panel::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use super::hooks::TemplatesDialogView;
use dioxus::prelude::*;

/// The templates dialog's own shell, shaped from its view: the open value driving
/// the backdrop, the change handler that writes the open signal, and the panel's own
/// domain values — its header title, the close handler, and the resolved card views.
pub(super) struct TemplatesDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) cards: Vec<TemplateCardView>,
}

impl From<&TemplatesDialogView> for TemplatesDialogShell {
    fn from(view: &TemplatesDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = String::from("Layout Templates");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let cards = view.cards.clone();
        Self {
            open,
            on_open_change,
            title,
            on_close,
            cards,
        }
    }
}
