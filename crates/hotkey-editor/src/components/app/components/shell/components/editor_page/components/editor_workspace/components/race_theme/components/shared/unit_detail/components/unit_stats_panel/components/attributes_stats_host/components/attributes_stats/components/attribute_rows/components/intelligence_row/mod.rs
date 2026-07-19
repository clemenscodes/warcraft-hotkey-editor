pub mod components;
mod model;
mod presentation;
mod view;

pub use view::IntelligenceRowView;

use components::primary_intelligence_row::PrimaryIntelligenceRow;
use components::regular_intelligence_row::RegularIntelligenceRow;
use dioxus::prelude::*;
use model::IntelligenceRowModel;
use presentation::{IntelligenceRowPresentation, use_intelligence_row};
use tw_macro::assert_component;

#[component]
pub fn IntelligenceRow(props: IntelligenceRowModel) -> Element {
    let IntelligenceRowPresentation {
        statistic,
        growth,
        label,
    } = use_intelligence_row(&props);
    if props.is_primary {
        rsx! {
            PrimaryIntelligenceRow {
                statistic,
                growth,
                label,
            }
        }
    } else {
        rsx! {
            RegularIntelligenceRow {
                statistic,
                growth,
                label,
            }
        }
    }
}

assert_component!(IntelligenceRow);
