pub mod components;
mod model;
mod view;

pub use view::HelpWorkflowSectionView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_callout::HelpCallout;
use dioxus::prelude::*;
use model::HelpWorkflowSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpWorkflowSection(props: HelpWorkflowSectionModel) -> Element {
    let steps = props.steps;
    rsx! {
        section {
            class: CLASS,
            HelpSectionTitle {
                title: "The workflow",
            }
            HelpCallout {
                steps,
            }
        }
    }
}

assert_component!(HelpWorkflowSection);
