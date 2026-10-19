pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CollisionKindTabView;
mod style;

use components::collision_kind_tab_count::CollisionKindTabCount;
use components::collision_kind_tab_label::CollisionKindTabLabel;
use dioxus::prelude::*;
use model::CollisionKindTabModel;
use presentation::CollisionKindTabPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindTab(props: CollisionKindTabModel) -> Element {
    let CollisionKindTabPresentation { label, current } =
        CollisionKindTabPresentation::from(&props);
    let text = label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": current,
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

assert_component!(CollisionKindTab);
