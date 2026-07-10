pub mod components;
mod props;
mod style;

use components::agility_row::AgilityRow;
use components::intelligence_row::IntelligenceRow;
use components::strength_row::StrengthRow;
use dioxus::prelude::*;
pub use props::AttributeRowsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hero's three attribute rows stacked beside the column's primary-attribute icon.
/// It names its rows directly — each attribute row owns its own look and its primary
/// glow.
#[component]
pub fn AttributeRows(props: AttributeRowsProps) -> Element {
    let strength = props.strength;
    let strength_is_primary = props.strength_is_primary;
    let agility = props.agility;
    let agility_is_primary = props.agility_is_primary;
    let intelligence = props.intelligence;
    let intelligence_is_primary = props.intelligence_is_primary;
    rsx! {
        div {
            class: CLASS,
            StrengthRow { statistic: strength, is_primary: strength_is_primary }
            AgilityRow { statistic: agility, is_primary: agility_is_primary }
            IntelligenceRow { statistic: intelligence, is_primary: intelligence_is_primary }
        }
    }
}

assert_component!(AttributeRows);
