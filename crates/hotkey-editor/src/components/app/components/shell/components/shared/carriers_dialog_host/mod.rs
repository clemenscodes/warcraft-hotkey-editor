pub mod components;
mod hooks;
mod style;

use crate::services::carriers::InspectedAbility;
use components::carriers_dialog::CarriersDialog;
use dioxus::prelude::*;
use hooks::use_carriers_dialog_host;
use style::CLASS;
use tw_macro::assert_component;

/// Connected wrapper for the carriers dialog: mounts it while `open_state` names an
/// ability, resolving that ability's carriers through the query. The trigger that opens
/// the dialog creates `open_state` and renders this host directly beneath itself — so
/// no page or other ancestor needs to know the dialog exists. Keeping the mount
/// conditional here re-initialises the dialog's own open signal each time it opens.
#[component]
pub fn CarriersDialogHost(open_state: Signal<Option<InspectedAbility>>) -> Element {
    let dialog = use_carriers_dialog_host(open_state);
    let Some(dialog) = dialog else {
        return rsx! {};
    };
    rsx! {
        div {
            class: CLASS,
            CarriersDialog { ..dialog }
        }
    }
}

assert_component!(CarriersDialogHost);
