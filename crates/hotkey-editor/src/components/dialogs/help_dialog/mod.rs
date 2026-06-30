pub mod components;
pub mod content;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::help_body::{HelpBody, HelpBodyProps};
use components::help_dismiss::{HelpDismiss, HelpDismissProps};
use content::HELP_CONTENT;

pub use props::HelpDialogProps;

assert_component!(HelpDialog);

/// The onboarding guide. Just a component that composes the `Dialog` base: it
/// sources the guide content, hands the body its data, and puts a dismiss button
/// in the footer slot. It renders no element of its own.
#[component]
pub fn HelpDialog(props: HelpDialogProps) -> Element {
    let dismiss = HelpDismissProps::from(&props);
    let body = HelpBodyProps {
        content: HELP_CONTENT,
    };
    rsx! {
        Dialog {
            open: props.help_open,
            title: "How to use this editor",
            footer: Some(rsx! {
                HelpDismiss { ..dismiss }
            }),
            HelpBody { ..body }
        }
    }
}
