pub mod components;
mod props;
mod style;

use components::help_body::{HelpBody, HelpBodyProps};
use components::help_dismiss::{HelpDismiss, HelpDismissProps};
use dioxus::prelude::*;
pub use props::HelpDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The help dialog's scrolling content region between the header and the panel
/// edge, holding the guide body and the dismiss button.
#[component]
pub fn HelpDialogBody(props: HelpDialogBodyProps) -> Element {
    let body = HelpBodyProps::from(&props);
    let dismiss = HelpDismissProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            HelpBody { ..body }
            HelpDismiss { ..dismiss }
        }
    }
}

assert_component!(HelpDialogBody);
