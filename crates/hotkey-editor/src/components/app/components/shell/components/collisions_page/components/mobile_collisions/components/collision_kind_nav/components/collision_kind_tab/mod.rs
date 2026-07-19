pub mod components;
mod model;
mod view;

pub use view::CollisionKindTabView;

use components::active_collision_kind_tab::ActiveCollisionKindTab;
use components::inactive_collision_kind_tab::InactiveCollisionKindTab;
use dioxus::prelude::*;
use model::CollisionKindTabModel;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindTab(props: CollisionKindTabModel) -> Element {
    match props.active {
        true => {
            let label = props.label.clone();
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
            let label = props.label.clone();
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
