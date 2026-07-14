pub mod components;
mod model;
mod view;

pub use view::HelpLegendView;
mod style;

use components::help_legend_row::HelpLegendRow;
use dioxus::prelude::*;
use model::HelpLegendModel;
use style::CLASS;
use tw_macro::assert_component;

/// The list of toolbar buttons with what each one does, one row per entry passed
/// in.
#[component]
pub fn HelpLegend(props: HelpLegendModel) -> Element {
    let rows = props.rows;
    rsx! {
        ul {
            class: CLASS,
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
