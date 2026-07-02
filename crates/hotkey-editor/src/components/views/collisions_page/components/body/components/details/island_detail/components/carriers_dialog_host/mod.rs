pub mod components;
mod props;

use components::carriers_dialog::{CarriersDialog, CarriersDialogProps};
use dioxus::prelude::*;
pub use props::CarriersDialogHostProps;

/// Mounts the carriers dialog while `carrier_dialog` names an ability, and nothing
/// when it is empty. Keeping the conditional mount here (not inside the dialog)
/// lets it re-open cleanly each time.
#[component]
pub fn CarriersDialogHost(props: CarriersDialogHostProps) -> Element {
    let carrier_dialog = props.carrier_dialog;
    let view_navigation = props.view_navigation;
    let dialog_state = carrier_dialog.read().clone();
    let Some(dialog_data) = dialog_state else {
        return rsx! {};
    };
    let dialog = CarriersDialogProps {
        dialog_data,
        carrier_dialog,
        view_navigation,
    };
    rsx! {
        CarriersDialog { ..dialog }
    }
}
