pub mod components;
mod hooks;
mod props;
mod view;

pub use view::IntelligenceRowView;

use components::primary_intelligence_row::PrimaryIntelligenceRow;
use components::regular_intelligence_row::RegularIntelligenceRow;
use dioxus::prelude::*;
use hooks::{IntelligenceRowModel, use_intelligence_row};
use props::IntelligenceRowProps;
use tw_macro::assert_component;

/// The hero's intelligence attribute row. A dispatcher: when intelligence is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn IntelligenceRow(props: IntelligenceRowProps) -> Element {
    let IntelligenceRowModel {
        statistic,
        growth,
        label,
    } = use_intelligence_row(&props);
    if props.is_primary {
        rsx! {
            PrimaryIntelligenceRow { statistic, growth, label }
        }
    } else {
        rsx! {
            RegularIntelligenceRow { statistic, growth, label }
        }
    }
}

assert_component!(IntelligenceRow);
