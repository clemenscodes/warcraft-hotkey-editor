mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpDismissProps;

assert_component!(HelpDismiss);

/// The footer button that closes the guide and records that the player has seen
/// it, so it stops auto-opening.
#[component]
pub fn HelpDismiss(props: HelpDismissProps) -> Element {
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick: props.on_dismiss,
            "Got it, don't show this again"
        }
    }
}
