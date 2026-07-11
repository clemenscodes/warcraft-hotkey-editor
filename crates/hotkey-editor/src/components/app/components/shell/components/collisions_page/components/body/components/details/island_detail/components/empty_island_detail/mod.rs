mod model;
mod style;
mod view;

pub use view::EmptyIslandDetailView;

use dioxus::prelude::*;
use model::EmptyIslandDetailModel;
use style::CLASS;
use tw_macro::assert_component;

/// The empty detail pane: the base pane surface, centered and muted, showing the prompt.
#[component]
pub fn EmptyIslandDetail(props: EmptyIslandDetailModel) -> Element {
    rsx! {
        section {
            class: CLASS,
            p { {props.prompt} }
        }
    }
}

assert_component!(EmptyIslandDetail);
