pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::collisions_clear_icon::CollisionsClearIcon;
use components::collisions_clear_label::CollisionsClearLabel;
use dioxus::prelude::*;
pub use props::CollisionsClearStateProps;
use style::CLASS;
assert_component!(CollisionsClearState);

/// The centered "all clear" state for a collision kind with no conflicts.
#[component]
pub fn CollisionsClearState(props: CollisionsClearStateProps) -> Element {
    let collision_kind = props.collision_kind;
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            CollisionsClearIcon {}
            CollisionsClearLabel {}
        }
    }
}
