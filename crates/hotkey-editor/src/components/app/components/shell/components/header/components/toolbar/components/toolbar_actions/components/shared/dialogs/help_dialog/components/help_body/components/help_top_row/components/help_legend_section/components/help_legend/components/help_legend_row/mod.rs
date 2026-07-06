pub mod components;
mod props;
mod style;

use components::help_legend_description::HelpLegendDescription;
use components::help_legend_icon::HelpLegendIcon;
use components::help_legend_label::HelpLegendLabel;
use dioxus::prelude::*;
pub use props::HelpLegendRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpLegendRow);

/// One legend entry: a framed glyph beside its label and description.
#[component]
pub fn HelpLegendRow(props: HelpLegendRowProps) -> Element {
    let icon = props.icon;
    let label = props.label;
    let description = props.description;
    rsx! {
        li { class: CLASS,
            HelpLegendIcon { icon }
            span {
                HelpLegendLabel { "{label}" }
                HelpLegendDescription { " {description}" }
            }
        }
    }
}
