pub mod components;
mod props;
mod style;

use components::help_workflow::HelpWorkflow;
use dioxus::prelude::*;
use props::HelpCalloutProps;
use style::CLASS;
use tw_macro::assert_component;

/// The framed panel that sets the workflow apart from the surrounding copy.
#[component]
pub fn HelpCallout(props: HelpCalloutProps) -> Element {
    let steps = props.steps;
    rsx! {
        div { class: CLASS,
            HelpWorkflow { steps }
        }
    }
}

assert_component!(HelpCallout);
