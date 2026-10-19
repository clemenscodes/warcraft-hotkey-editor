pub mod components;
mod model;
mod style;
mod view;

pub use view::EditorWorkspaceView;

use components::race_theme::RaceTheme;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorWorkspace() -> Element {
    let class = CLASS;
    rsx! {
        div {
            class,
            RaceTheme {}
        }
    }
}

assert_component!(EditorWorkspace);
