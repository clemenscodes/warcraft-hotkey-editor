pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_legend::{HelpLegend, HelpLegendProps};
use dioxus::prelude::*;
pub use props::HelpLegendSectionProps;
use style::CLASS;
assert_component!(HelpLegendSection);

/// The right column of the top row: the heading above the toolbar button legend.
#[component]
pub fn HelpLegendSection(props: HelpLegendSectionProps) -> Element {
    let legend = HelpLegendProps::from(&props);
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { "Button legend" }
            HelpLegend { ..legend }
        }
    }
}
