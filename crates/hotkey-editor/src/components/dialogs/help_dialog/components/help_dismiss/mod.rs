mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HelpDismissProps;
use style::CLASS;
assert_component!(HelpDismiss);

/// The footer button that closes the guide and records that the player has seen
/// it, so it stops auto-opening.
#[component]
pub fn HelpDismiss(props: HelpDismissProps) -> Element {
    rsx! {
        button { class: CLASS, r#type: "button", onclick: props.on_dismiss,
            "Got it, don't show this again"
        }
    }
}
