use super::components::carriers_dialog_body::CarriersDialogBodyProps;
use super::components::carriers_dialog_body::components::carriers_grid::components::carrier_card::CarrierCardProps;
use super::hooks::CarriersDialogView;
use super::props::CarriersDialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// One card per resolved carrier of the ability.
pub(super) fn cards(props: &CarriersDialogProps) -> Vec<CarrierCardProps> {
    props
        .carriers
        .iter()
        .map(|carrier| CarrierCardProps {
            unit_id: carrier.unit_id(),
            icon_url: carrier.icon_url().map(str::to_owned),
            name: carrier.name().to_owned(),
        })
        .collect()
}

/// The carriers dialog's own shell, shaped from its view: the open value driving the
/// backdrop, the change handler that writes the open signal, the header props, and
/// the scroll-region body props carrying the carrier cards.
pub(super) struct CarriersDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) header: DialogHeaderProps,
    pub(super) body: CarriersDialogBodyProps,
}

impl From<&CarriersDialogView> for CarriersDialogShell {
    fn from(view: &CarriersDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = view.title.clone();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let cards = view.cards.clone();
        let body = CarriersDialogBodyProps { cards };
        Self {
            open,
            on_open_change,
            header,
            body,
        }
    }
}
