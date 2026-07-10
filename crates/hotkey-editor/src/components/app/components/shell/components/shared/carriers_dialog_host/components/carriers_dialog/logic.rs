use super::hooks::CarriersDialogView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// The carriers dialog's own shell, shaped from its view: the open value driving the
/// backdrop, the change handler that writes the open signal, the close handler its header
/// fires, the title, and the carriers the panel lays out below.
pub(super) struct CarriersDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) carriers: Vec<CarrierUnitView>,
}

impl From<&CarriersDialogView> for CarriersDialogShell {
    fn from(view: &CarriersDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let title = view.title.clone();
        let carriers = view.carriers.clone();
        Self {
            open,
            on_open_change,
            title,
            on_close,
            carriers,
        }
    }
}
