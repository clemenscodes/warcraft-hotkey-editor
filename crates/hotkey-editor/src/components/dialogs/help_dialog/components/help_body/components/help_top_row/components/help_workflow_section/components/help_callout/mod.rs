pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::help_workflow::{HelpWorkflow, HelpWorkflowProps};
use style::CLASS;

pub use props::HelpCalloutProps;

assert_component!(HelpCallout);

/// The framed panel that sets the workflow apart from the surrounding copy.
#[component]
pub fn HelpCallout(props: HelpCalloutProps) -> Element {
    let workflow = HelpWorkflowProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            HelpWorkflow { ..workflow }
        }
    }
}
