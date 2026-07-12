pub mod components;
mod model;
mod view;

pub use view::HelpResolverSectionView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_glossary_columns::HelpGlossaryColumns;
use components::help_resolver_prose::HelpResolverProse;
use dioxus::prelude::*;
use model::HelpResolverSectionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The full-width lower region: the heading, the prose walkthrough of what the
/// resolver does, and the glossary that defines its terms.
#[component]
pub fn HelpResolverSection(props: HelpResolverSectionModel) -> Element {
    let paragraphs = props.prose;
    let columns = props.glossary;
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { title: "What the resolver is doing" }
            HelpResolverProse { paragraphs }
            HelpGlossaryColumns { columns }
        }
    }
}

assert_component!(HelpResolverSection);
