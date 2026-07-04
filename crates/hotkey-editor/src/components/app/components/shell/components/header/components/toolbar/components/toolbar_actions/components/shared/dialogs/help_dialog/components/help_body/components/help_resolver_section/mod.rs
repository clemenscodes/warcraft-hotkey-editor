pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_glossary_columns::{HelpGlossaryColumns, HelpGlossaryColumnsProps};
use components::help_resolver_prose::{HelpResolverProse, HelpResolverProseProps};
use dioxus::prelude::*;
pub use props::HelpResolverSectionProps;
use style::CLASS;
assert_component!(HelpResolverSection);

/// The full-width lower region: the heading, the prose walkthrough of what the
/// resolver does, and the glossary that defines its terms.
#[component]
pub fn HelpResolverSection(props: HelpResolverSectionProps) -> Element {
    let prose = HelpResolverProseProps::from(&props);
    let glossary = HelpGlossaryColumnsProps::from(&props);
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { "What the resolver is doing" }
            HelpResolverProse { ..prose }
            HelpGlossaryColumns { ..glossary }
        }
    }
}
