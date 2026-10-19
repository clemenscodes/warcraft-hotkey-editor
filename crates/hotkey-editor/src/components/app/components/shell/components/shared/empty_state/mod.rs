pub mod components;
mod model;
mod view;

pub use view::EmptyStateView;
mod style;

use components::empty_message::EmptyMessage;
use dioxus::prelude::*;
use model::EmptyStateModel;
use style::CLASS;
use tw_macro::assert_component;

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
