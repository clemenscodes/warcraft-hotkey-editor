mod props;
mod style;

use crate::components::app::components::shell::components::shared::empty_message::EmptyMessage;
use crate::components::app::components::shell::components::shared::page_state::PageState;
use dioxus::prelude::*;
pub use props::EmptyStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyState);

/// The centered upload prompt for a collision kind with no file loaded. A thin
/// identity wrapper that tags the collision kind for e2e and hands the shared
/// `PageState` shell its prompt message.
#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    let collision_kind = props.collision_kind.kind_param();
    let message = props.message;
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            PageState {
                EmptyMessage { text: message }
            }
        }
    }
}
