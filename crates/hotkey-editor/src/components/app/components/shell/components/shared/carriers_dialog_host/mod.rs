pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use crate::services::carriers::InspectedAbility;
use components::carriers_dialog_body::CarriersDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::OpenCarriersDialog;
use presentation::use_carriers_dialog_host;
use style::CLASS;
use tw_macro::assert_component;

/// Connected wrapper for the carriers dialog: mounts it while `open_state` names an
/// ability, resolving that ability's carriers through the query. The trigger that opens
/// the dialog creates `open_state` and renders this host directly beneath itself — so no
/// page or other ancestor needs to know the dialog exists. It renders the reusable
/// `WarcraftDialog`, handing it the isolated carriers grid as its body region; keeping the
/// mount conditional here re-initialises the headless dialog each time it opens.
#[component]
pub fn CarriersDialogHost(open_state: Signal<Option<InspectedAbility>>) -> Element {
    let dialog = use_carriers_dialog_host(open_state);
    let Some(dialog) = dialog else {
        return rsx! {};
    };
    let OpenCarriersDialog {
        title,
        carriers,
        on_open_change,
    } = dialog;
    let body = CarriersDialogBodyView { carriers };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<CarriersDialogBodyView, Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(CarriersDialogHost);
