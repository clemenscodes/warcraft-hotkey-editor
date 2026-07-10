pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_legend::{HelpLegend, HelpLegendProps};
use dioxus::prelude::*;
pub use props::HelpLegendSectionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The right column of the top row: the heading above the toolbar button legend.
#[component]
pub fn HelpLegendSection(props: HelpLegendSectionProps) -> Element {
    let legend = HelpLegendProps::from(&props);
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { title: "Button legend" }
            HelpLegend { ..legend }
        }
    }
}

assert_component!(HelpLegendSection);
