pub mod components;
mod model;
mod style;
mod view;

pub use view::EditorTabsBarView;

use components::race_tabs_host::RaceTabsHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorTabsBar() -> Element {
    rsx! {
        div {
            class: CLASS,
            RaceTabsHost {}
        }
    }
}

assert_component!(EditorTabsBar);
