use super::props::CarriersDialogProps;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// The dialog's shaped state: the open signal it drives, the title, and the resolved
/// carriers. Owns the effect that clears the trigger's open-state when the dialog closes,
/// so the body only names the result and renders.
pub(super) struct CarriersDialogView {
    pub(super) open: Signal<bool>,
    pub(super) title: String,
    pub(super) carriers: Vec<CarrierUnitView>,
}

pub(super) fn use_carriers_dialog(props: &CarriersDialogProps) -> CarriersDialogView {
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
