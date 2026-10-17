pub mod components;
mod model;
mod presentation;
mod view;

pub use view::TemplatePagerView;
mod style;

use components::template_pager_track::TemplatePagerTrack;
use dioxus::prelude::*;
use model::TemplatePagerModel;
use presentation::{TemplatePagerPresentation, use_template_pager};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplatePager(props: TemplatePagerModel) -> Element {
    let TemplatePagerPresentation { cards, onmounted } = use_template_pager(&props);
    rsx! {
        section {
            class: CLASS,
            aria_label: "Layout templates",
            tabindex: "-1",
            onmounted: move |event| onmounted.call(event),
            TemplatePagerTrack {
                cards,
            }
        }
    }
}

assert_component!(TemplatePager);
