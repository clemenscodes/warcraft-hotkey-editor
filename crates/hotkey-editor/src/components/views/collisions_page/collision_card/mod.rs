mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionCardProps;
use style::CLASS;
assert_component!(CollisionCard);

/// A selectable collision-list card.
#[component]
pub fn CollisionCard(props: CollisionCardProps) -> Element {
    let is_selected = props.is_selected;
    let collision_key = props.collision_key;
    let onclick = props.onclick;
    let children = props.children;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-selected": is_selected,
            "data-collision-key": collision_key,
            onclick,
            {children}
        }
    }
}
