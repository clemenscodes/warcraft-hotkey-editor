pub mod components;
mod style;

use components::collisions_button_host::CollisionsButtonHost;
use components::search_button_host::SearchButtonHost;
use components::toolbar_actions::ToolbarActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Editor actions",
            CollisionsButtonHost {}
            SearchButtonHost {}
            ToolbarActions {}
        }
    }
}

assert_component!(Toolbar);
