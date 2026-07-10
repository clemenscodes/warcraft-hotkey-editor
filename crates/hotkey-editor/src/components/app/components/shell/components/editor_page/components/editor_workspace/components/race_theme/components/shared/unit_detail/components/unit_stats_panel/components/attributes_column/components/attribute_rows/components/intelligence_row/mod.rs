pub mod components;
mod props;

use components::primary_intelligence_row::{PrimaryIntelligenceRow, PrimaryIntelligenceRowProps};
use components::regular_intelligence_row::{RegularIntelligenceRow, RegularIntelligenceRowProps};
use dioxus::prelude::*;
pub use props::IntelligenceRowProps;
use tw_macro::assert_component;

/// The hero's intelligence attribute row. A dispatcher: when intelligence is the hero's primary
/// attribute it renders the glowing primary row, otherwise the resting regular row — each
/// owns its own look, so there is no `data-primary` attribute.
#[component]
pub fn IntelligenceRow(props: IntelligenceRowProps) -> Element {
    if props.is_primary {
        let row = PrimaryIntelligenceRowProps::from(&props);
        rsx! {
            PrimaryIntelligenceRow { ..row }
        }
    } else {
        let row = RegularIntelligenceRowProps::from(&props);
        rsx! {
            RegularIntelligenceRow { ..row }
        }
    }
}

assert_component!(IntelligenceRow);
