pub mod components;
pub mod route;

use crate::components::app::route::Route;
use dioxus::prelude::*;
use tw_macro::assert_component;

/// The editor's compiled Tailwind stylesheet. Public so the component gallery can
/// inject the same asset and render the editor's components with their real styling.
pub const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

/// The application root. It mounts the Dioxus `Router`, whose [`Route`] enum nests
/// the editor, collisions, and resolve pages under one shared
/// [`Shell`](components::shell::Shell) layout — so the router owns history and URL
/// synchronisation, and the shell owns the app-wide state that persists as the pages
/// swap beneath it.
#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

assert_component!(App);
