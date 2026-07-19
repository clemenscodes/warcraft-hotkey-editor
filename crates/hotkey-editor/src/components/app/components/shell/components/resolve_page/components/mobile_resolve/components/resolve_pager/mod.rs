pub mod components;
mod model;
mod view;

pub use view::ResolvePagerView;
mod style;

use components::resolve_pager_cards::ResolvePagerCards;
use dioxus::prelude::*;
use model::ResolvePagerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolvePager(props: ResolvePagerModel) -> Element {
    let section = props.section;
    let unresolved = props.unresolved;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Resolve pager",
            ResolvePagerCards {
                section,
                unresolved,
            }
        }
    }
}

assert_component!(ResolvePager);
