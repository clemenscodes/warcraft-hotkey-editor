pub mod components;
pub mod route;

use crate::components::app::route::Route;
use dioxus::prelude::*;
use tw_macro::assert_component;

const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

assert_component!(App);
