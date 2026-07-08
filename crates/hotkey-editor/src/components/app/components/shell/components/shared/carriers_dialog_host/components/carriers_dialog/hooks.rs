use super::components::carriers_dialog_body::components::carriers_grid::components::carrier_card::CarrierCardProps;
use super::logic::cards;
use super::props::CarriersDialogProps;
use dioxus::prelude::*;

/// The dialog's shaped state: the open signal it drives, the title, and the carrier
/// cards. Owns the effect that clears the trigger's open-state when the dialog closes,
/// so the body only names the result and renders.
pub(super) struct CarriersDialogView {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) cards: Vec<CarrierCardProps>,
}

pub(super) fn use_carriers_dialog(props: &CarriersDialogProps) -> CarriersDialogView {
    let mut open_state = props.open_state;
    let open = use_signal(|| true);
    use_effect(move || {
        if !open() {
            open_state.set(None);
        }
    });
    let cards = cards(props);
    let title = props.title.clone();
    CarriersDialogView { open, title, cards }
}
