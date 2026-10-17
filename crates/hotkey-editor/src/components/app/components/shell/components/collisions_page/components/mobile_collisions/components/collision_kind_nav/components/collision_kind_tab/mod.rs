pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CollisionKindTabView;

use components::active_collision_kind_tab::ActiveCollisionKindTab;
use components::inactive_collision_kind_tab::InactiveCollisionKindTab;
use dioxus::prelude::*;
use model::CollisionKindTabModel;
use presentation::CollisionKindTabPresentation;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindTab(props: CollisionKindTabModel) -> Element {
    let CollisionKindTabPresentation { label } = CollisionKindTabPresentation::from(&props);
    match props.active {
        true => {
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                ActiveCollisionKindTab {
                    label,
                    count,
                    onclick,
                }
            }
        }
        false => {
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                InactiveCollisionKindTab {
                    label,
                    count,
                    onclick,
                }
            }
        }
    }
}

assert_component!(CollisionKindTab);
