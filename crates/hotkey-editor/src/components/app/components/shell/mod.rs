pub mod components;
mod hooks;
mod logic;
mod route_sync;
mod style;

use crate::components::app::route::Route;
use components::document_head::DocumentHead;
use components::footer::Footer;
use components::header::Header;
use components::toasts::Toasts;
use dioxus::prelude::*;
use hooks::{ShellModel, use_shell};
use tw_macro::assert_component;

assert_component!(Shell);

/// The persistent application frame: the header, the routed page (via `Outlet`), the
/// footer, and the overlay dialogs, wrapped in the toast provider and the app-root
/// element that owns the page's background, typography, and scrollbar.
///
/// This is the Dioxus **layout** shared by the three page routes, so it stays mounted
/// while the `Outlet` swaps the active page — which is what lets every signal
/// [`use_shell`] owns (the loaded keys, grid layout, editor selection) survive
/// navigation between the editor, collisions, and resolve pages. It replaces the old
/// one-page `Workbench` god-component: its body is a flat list of children, every
/// piece of app-wide state reaching the pages and header through context rather than
/// props.
#[component]
pub fn Shell() -> Element {
    let ShellModel {
        class,
        handle_keydown,
    } = use_shell();
    rsx! {
        DocumentHead {}
        Toasts {
            div {
                class,
                onkeydown: handle_keydown,
                Header {}
                Outlet::<Route> {}
                Footer {}
            }
        }
    }
}
