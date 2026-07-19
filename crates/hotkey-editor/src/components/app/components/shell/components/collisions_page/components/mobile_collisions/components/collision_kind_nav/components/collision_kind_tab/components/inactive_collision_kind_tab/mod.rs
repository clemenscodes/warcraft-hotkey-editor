mod model;
mod view;

pub use view::InactiveCollisionKindTabView;
mod style;

use super::shared::collision_kind_tab_count::CollisionKindTabCount;
use super::shared::collision_kind_tab_label::CollisionKindTabLabel;
use dioxus::prelude::*;
use model::InactiveCollisionKindTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InactiveCollisionKindTab(props: InactiveCollisionKindTabModel) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "false",
            onclick,
            CollisionKindTabLabel {
                text,
            }
            CollisionKindTabCount {
                count,
            }
        }
    }
}

assert_component!(InactiveCollisionKindTab);
