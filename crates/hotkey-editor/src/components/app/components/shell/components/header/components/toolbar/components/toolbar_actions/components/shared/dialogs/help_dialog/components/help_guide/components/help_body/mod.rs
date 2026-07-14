pub mod components;
mod model;
mod view;

pub use view::HelpBodyView;
mod style;

use components::help_resolver_section::HelpResolverSection;
use components::help_top_row::HelpTopRow;
use dioxus::prelude::*;
use model::HelpBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpBody(props: HelpBodyModel) -> Element {
    let content = props.content;
    let prose = content.resolver_prose();
    let glossary = content.glossary();
    rsx! {
        div {
            class: CLASS,
            HelpTopRow {
                content,
            }
            HelpResolverSection {
                prose,
                glossary,
            }
        }
    }
}

assert_component!(HelpBody);
