mod props;
mod style;

use crate::components::app::components::shell::components::shared::empty_message::EmptyMessage;
use dioxus::prelude::*;
pub use props::EmptyStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyState);

/// The centered upload prompt for a collision kind with no file loaded. It centers its
/// prompt message in the available space and tags the collision kind for e2e.
#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    let collision_kind = props.collision_kind.kind_param();
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
