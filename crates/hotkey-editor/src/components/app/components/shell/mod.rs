pub mod components;
mod presentation;
mod style;

use crate::components::app::route::Route;
use components::footer::Footer;
use components::head::Head;
use components::header::Header;
use components::toasts::{ToastOverlay, use_toast_provider};
use dioxus::prelude::*;
use presentation::{ShellModel, use_shell};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Shell() -> Element {
    let ShellModel { handle_keydown } = use_shell();
    let toast_model = use_toast_provider();
    let toasts = toast_model.records();
    let on_remove = toast_model.on_remove();
    rsx! {
        Head {}
        div {
            class: CLASS,
            onkeydown: handle_keydown,
            Header {}
            Outlet::<Route> {}
            Footer {}
        }
        ToastOverlay {
            toasts,
            on_remove,
        }
    }
}

assert_component!(Shell);
