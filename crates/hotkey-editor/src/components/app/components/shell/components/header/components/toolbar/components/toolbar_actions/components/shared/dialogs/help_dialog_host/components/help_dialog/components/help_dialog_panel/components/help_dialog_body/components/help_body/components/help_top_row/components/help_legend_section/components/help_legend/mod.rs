pub mod components;
mod props;
mod style;

use components::help_legend_row::HelpLegendRow;
use dioxus::prelude::*;
use props::HelpLegendProps;
use style::CLASS;
use tw_macro::assert_component;

/// The list of toolbar buttons with what each one does, one row per entry passed
/// in.
#[component]
pub fn HelpLegend(props: HelpLegendProps) -> Element {
    let rows = props.rows;
    rsx! {
        ul { class: CLASS,
            for (index, entry) in rows.iter().copied().enumerate() {
                HelpLegendRow {
                    key: "{index}",
                    entry,
                }
            }
        }
    }
}

assert_component!(HelpLegend);
