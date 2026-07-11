mod model;
mod style;
mod view;

pub use view::EmptyUnitPositionDetailView;

use dioxus::prelude::*;
use model::EmptyUnitPositionDetailModel;
use style::CLASS;
use tw_macro::assert_component;

/// The empty detail pane: the base pane surface, centered and muted, showing the prompt.
#[component]
pub fn EmptyUnitPositionDetail(props: EmptyUnitPositionDetailModel) -> Element {
    rsx! {
        section {
            class: CLASS,
            p { {props.prompt} }
        }
    }
}

assert_component!(EmptyUnitPositionDetail);
