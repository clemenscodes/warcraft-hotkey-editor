pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_section_title::HelpSectionTitle;
use components::help_legend::{HelpLegend, HelpLegendProps};
use style::CLASS;

pub use props::HelpLegendSectionProps;

assert_component!(HelpLegendSection);

/// The right column of the top row: the heading above the toolbar button legend.
#[component]
pub fn HelpLegendSection(props: HelpLegendSectionProps) -> Element {
    let legend = HelpLegendProps::from(&props);
    rsx! {
        section {
            class: CLASS,
            HelpSectionTitle { "Button legend" }
            HelpLegend { ..legend }
        }
    }
}
