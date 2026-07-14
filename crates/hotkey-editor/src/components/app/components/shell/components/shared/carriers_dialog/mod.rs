pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::CarriersDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::carriers_dialog_body::CarriersDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::CarriersDialogModel;
use presentation::OpenCarriersDialog;
use presentation::use_carriers_dialog;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CarriersDialog(props: CarriersDialogModel) -> Element {
    let dialog = use_carriers_dialog(&props);
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
            WarcraftDialog::<CarriersDialogBodyView,Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(CarriersDialog);
