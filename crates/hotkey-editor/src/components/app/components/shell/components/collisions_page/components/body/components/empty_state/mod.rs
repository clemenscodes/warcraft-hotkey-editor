mod model;
mod view;

pub use view::EmptyStateView;
mod style;

use crate::components::app::components::shell::components::shared::empty_message::EmptyMessage;
use dioxus::prelude::*;
use model::EmptyStateModel;
use style::CLASS;
use tw_macro::assert_component;

/// The centered upload prompt for a collision kind with no file loaded. It centers its
/// prompt message in the available space.
#[component]
pub fn EmptyState(props: EmptyStateModel) -> Element {
    let message = props.message;
    rsx! {
        section {
            class: CLASS,
            EmptyMessage {
                text: message,
            }
        }
    }
}

assert_component!(EmptyState);
