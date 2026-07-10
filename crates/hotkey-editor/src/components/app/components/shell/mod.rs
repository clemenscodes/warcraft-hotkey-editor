pub mod components;
mod hooks;
mod logic;
mod route_sync;
mod style;

use crate::components::app::route::Route;
use components::footer::Footer;
use components::head::Head;
use components::header::Header;
use components::toasts::{ToastContainer, ToastContainerProps, use_toast_provider};
use dioxus::prelude::*;
use hooks::{ShellModel, use_shell};
use style::CLASS;
use tw_macro::assert_component;

/// The persistent application frame: the header, the routed page (via `Outlet`), the
/// footer, and the overlay dialogs, wrapped in the toast provider and the app-root
/// element that owns the page's background, typography, and scrollbar.
///
/// This is the Dioxus **layout** shared by the three page routes, so it stays mounted
/// while the `Outlet` swaps the active page — which is what lets every signal
/// [`use_shell`] owns (the loaded keys, grid layout, editor selection) survive
/// navigation between the editor, collisions, and resolve pages. It provides the toast
/// queue to its subtree and renders the fixed toast overlay itself, so no `children`
/// wrapper carries pre-rendered markup.
#[component]
pub fn Shell() -> Element {
    let ShellModel { handle_keydown } = use_shell();
    let toast_model = use_toast_provider();
    let toast_container = ToastContainerProps::from(&toast_model);
    rsx! {
        Head {}
        div {
            class: CLASS,
            onkeydown: handle_keydown,
            Header {}
            Outlet::<Route> {}
            Footer {}
        }
        ToastContainer { ..toast_container }
    }
}

assert_component!(Shell);
