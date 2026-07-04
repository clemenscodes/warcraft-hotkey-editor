pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::empty_message::EmptyMessage;
use dioxus::prelude::*;
pub use props::EmptyStateProps;
use style::CLASS;
assert_component!(EmptyState);

/// The centered upload prompt for a collision kind with no file loaded.
#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    let collision_kind = props.collision_kind;
    let message = props.message;
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            EmptyMessage { text: message }
        }
    }
}
