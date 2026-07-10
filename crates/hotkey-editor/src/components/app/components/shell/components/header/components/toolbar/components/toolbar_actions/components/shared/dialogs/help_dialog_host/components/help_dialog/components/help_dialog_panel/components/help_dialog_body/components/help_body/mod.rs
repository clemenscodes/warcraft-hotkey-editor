pub mod components;
mod props;
mod style;

use components::help_resolver_section::{HelpResolverSection, HelpResolverSectionProps};
use components::help_top_row::{HelpTopRow, HelpTopRowProps};
use dioxus::prelude::*;
pub use props::HelpBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The help guide's scrolling content: the split top row above the full-width
/// resolver explanation. Pure layout; it threads the content down.
#[component]
pub fn HelpBody(props: HelpBodyProps) -> Element {
    let top_row = HelpTopRowProps::from(&props);
    let resolver = HelpResolverSectionProps::from(&props);
    rsx! {
        div { class: CLASS,
            HelpTopRow { ..top_row }
            HelpResolverSection { ..resolver }
        }
    }
}

assert_component!(HelpBody);
