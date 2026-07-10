pub mod components;
mod props;
mod style;

use components::help_resolver_section::HelpResolverSection;
use components::help_top_row::HelpTopRow;
use dioxus::prelude::*;
use props::HelpBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The help guide's scrolling content: the split top row above the full-width
/// resolver explanation. Pure layout; it threads the content down.
#[component]
pub fn HelpBody(props: HelpBodyProps) -> Element {
    let content = props.content;
    let prose = content.resolver_prose();
    let glossary = content.glossary();
    rsx! {
        div { class: CLASS,
            HelpTopRow { content }
            HelpResolverSection {
                prose,
                glossary,
            }
        }
    }
}

assert_component!(HelpBody);
