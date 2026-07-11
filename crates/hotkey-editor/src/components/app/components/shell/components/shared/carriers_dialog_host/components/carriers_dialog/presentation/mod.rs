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
use super::model::CarriersDialogModel;

/// The dialog's shaped state: the open signal it drives, the title, and the resolved
/// carriers. Owns the effect that clears the trigger's open-state when the dialog closes,
/// so the body only names the result and renders.
pub(super) struct CarriersDialogView {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) carriers: Vec<CarrierUnitView>,
}

pub(super) fn use_carriers_dialog(props: &CarriersDialogModel) -> CarriersDialogView {
    let mut open_state = props.open_state;
    let open = use_signal(|| true);
    use_effect(move || {
        if !open() {
            open_state.set(None);
        }
    });
    let title = props.title.clone();
    let carriers = props.carriers.clone();
    CarriersDialogView {
        open,
        title,
        carriers,
    }
}

impl ddd::Presentation for CarriersDialogView {
    type Model = CarriersDialogModel;
}
