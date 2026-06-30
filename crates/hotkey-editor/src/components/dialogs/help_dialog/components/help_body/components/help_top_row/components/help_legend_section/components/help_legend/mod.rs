pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::help_legend_row::HelpLegendRow;
use style::CLASS;

pub use props::HelpLegendProps;

assert_component!(HelpLegend);

/// The list of toolbar buttons with what each one does, one row per entry passed
/// in.
#[component]
pub fn HelpLegend(props: HelpLegendProps) -> Element {
    rsx! {
        ul {
            class: CLASS,
            for row in props.rows.iter().cloned() {
                HelpLegendRow {
                    key: "{row.label}",
                    ..row
                }
            }
        }
    }
}
