pub mod components;
mod model;
mod presentation;
mod view;

pub use view::ResolvePagerView;
mod style;

use components::resolve_pager_cards::ResolvePagerCards;
use dioxus::prelude::*;
use model::ResolvePagerModel;
use presentation::{ResolvePagerPresentation, use_resolve_pager};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolvePager(props: ResolvePagerModel) -> Element {
    let ResolvePagerPresentation { onmounted } = use_resolve_pager(&props);
    let section = props.section;
    let unresolved = props.unresolved;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Resolve pager",
            tabindex: "-1",
            onmounted: move |event| onmounted.call(event),
            ResolvePagerCards {
                section,
                unresolved,
            }
        }
    }
}

assert_component!(ResolvePager);
