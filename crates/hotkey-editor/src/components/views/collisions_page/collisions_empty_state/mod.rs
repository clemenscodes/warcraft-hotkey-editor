pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::collisions_empty_message::CollisionsEmptyMessage;
use dioxus::prelude::*;
pub use props::CollisionsEmptyStateProps;
use style::CLASS;
assert_component!(CollisionsEmptyState);

/// The centered upload prompt for a collision kind with no file loaded.
#[component]
pub fn CollisionsEmptyState(props: CollisionsEmptyStateProps) -> Element {
    let collision_kind = props.collision_kind;
    let message = props.message;
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            CollisionsEmptyMessage { text: message }
        }
    }
}
