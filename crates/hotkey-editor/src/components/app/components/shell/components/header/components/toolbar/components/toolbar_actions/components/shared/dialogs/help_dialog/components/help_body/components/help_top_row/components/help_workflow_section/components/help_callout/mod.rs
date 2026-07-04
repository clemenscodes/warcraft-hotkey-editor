pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::help_workflow::{HelpWorkflow, HelpWorkflowProps};
use dioxus::prelude::*;
pub use props::HelpCalloutProps;
use style::CLASS;
assert_component!(HelpCallout);

/// The framed panel that sets the workflow apart from the surrounding copy.
#[component]
pub fn HelpCallout(props: HelpCalloutProps) -> Element {
    let workflow = HelpWorkflowProps::from(&props);
    rsx! {
        div { class: CLASS,
            HelpWorkflow { ..workflow }
        }
    }
}
