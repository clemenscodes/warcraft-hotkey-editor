mod props;

use super::carriers_dialog::{CarriersDialog, CarriersDialogProps};
use dioxus::prelude::*;
pub use props::CarriersDialogHostProps;

/// Mounts the carriers dialog while `carriers_dialog` names an ability, and
/// renders nothing when it is empty. Keeping the mount conditional here (rather
/// than guarding inside the dialog) lets the dialog re-open cleanly every time:
/// its open-state signal re-initialises on each fresh mount.
#[component]
pub fn CarriersDialogHost(props: CarriersDialogHostProps) -> Element {
    let carriers_dialog = props.carriers_dialog;
    let view_navigation = props.view_navigation;
    let dialog_state = carriers_dialog.read().clone();
    let Some(dialog_data) = dialog_state else {
        return rsx! {};
    };
    let dialog = CarriersDialogProps {
        dialog_data,
        carriers_dialog,
        view_navigation,
    };
    rsx! {
        CarriersDialog { ..dialog }
    }
}
