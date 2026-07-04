pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::help_legend_row::HelpLegendRow;
use dioxus::prelude::*;
pub use props::HelpLegendProps;
use style::CLASS;
assert_component!(HelpLegend);

/// The list of toolbar buttons with what each one does, one row per entry passed
/// in.
#[component]
pub fn HelpLegend(props: HelpLegendProps) -> Element {
    rsx! {
        ul { class: CLASS,
            for row in props.rows.iter().cloned() {
                HelpLegendRow { key: "{row.label}", ..row }
            }
        }
    }
}
