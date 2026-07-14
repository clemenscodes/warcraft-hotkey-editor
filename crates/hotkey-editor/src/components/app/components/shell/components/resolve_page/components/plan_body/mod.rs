pub mod components;
mod model;
mod view;

pub use view::PlanBodyView;
mod style;

use components::move_list::MoveList;
use components::unresolved_section::UnresolvedSection;
use dioxus::prelude::*;
use model::PlanBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PlanBody(props: PlanBodyModel) -> Element {
    let section = props.section;
    let unresolved = props.unresolved;
    let has_unresolved = !unresolved.is_empty();
    rsx! {
        div {
            class: CLASS,
            MoveList {
                section,
            }
            if has_unresolved {
                UnresolvedSection {
                    unresolved,
                }
            }
        }
    }
}

assert_component!(PlanBody);
