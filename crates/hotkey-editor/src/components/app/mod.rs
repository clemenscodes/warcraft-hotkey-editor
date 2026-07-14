pub mod components;
pub mod route;

use crate::components::app::route::Route;
use dioxus::prelude::*;
use tw_macro::assert_component;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {



        }
    }
}

assert_component!(App);
