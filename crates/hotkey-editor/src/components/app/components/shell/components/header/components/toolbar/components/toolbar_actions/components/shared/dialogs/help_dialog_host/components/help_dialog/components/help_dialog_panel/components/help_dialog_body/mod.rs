pub mod components;
mod props;
mod style;

use components::help_body::HelpBody;
use components::help_dismiss::HelpDismiss;
use dioxus::prelude::*;
use props::HelpDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The help dialog's scrolling content region between the header and the panel
/// edge, holding the guide body and the dismiss button.
#[component]
pub fn HelpDialogBody(props: HelpDialogBodyProps) -> Element {
    let content = props.content;
    let on_dismiss = props.on_dismiss;
    rsx! {
        div {
            class: CLASS,
            HelpBody { content }
            HelpDismiss { on_dismiss }
        }
    }
}

assert_component!(HelpDialogBody);
