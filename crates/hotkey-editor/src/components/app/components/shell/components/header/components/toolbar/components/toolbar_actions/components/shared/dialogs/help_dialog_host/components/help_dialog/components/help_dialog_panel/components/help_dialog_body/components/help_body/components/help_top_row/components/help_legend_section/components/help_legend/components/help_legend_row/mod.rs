pub mod components;
mod props;
mod style;

use components::help_legend_description::HelpLegendDescription;
use components::help_legend_icon::HelpLegendIcon;
use components::help_legend_label::HelpLegendLabel;
use dioxus::prelude::*;
use props::HelpLegendRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One legend entry: a framed glyph beside its label and description.
#[component]
pub fn HelpLegendRow(props: HelpLegendRowProps) -> Element {
    let entry = props.entry;
    let icon = entry.icon();
    let label = entry.label();
    let description = entry.description();
    rsx! {
        li { class: CLASS,
            HelpLegendIcon { icon }
            span {
                HelpLegendLabel { label }
                HelpLegendDescription { description }
            }
        }
    }
}

assert_component!(HelpLegendRow);
