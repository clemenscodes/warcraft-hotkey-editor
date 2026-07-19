pub mod components;
mod model;
mod view;

pub use view::AttributeRowsView;
mod style;

use components::agility_row::AgilityRow;
use components::intelligence_row::IntelligenceRow;
use components::strength_row::StrengthRow;
use dioxus::prelude::*;
use model::AttributeRowsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AttributeRows(props: AttributeRowsModel) -> Element {
    let strength = props.strength;
    let strength_is_primary = props.strength_is_primary;
    let agility = props.agility;
    let agility_is_primary = props.agility_is_primary;
    let intelligence = props.intelligence;
    let intelligence_is_primary = props.intelligence_is_primary;
    rsx! {
        div {
            class: CLASS,
            StrengthRow {
                statistic: strength,
                is_primary: strength_is_primary,
            }
            AgilityRow {
                statistic: agility,
                is_primary: agility_is_primary,
            }
            IntelligenceRow {
                statistic: intelligence,
                is_primary: intelligence_is_primary,
            }
        }
    }
}

assert_component!(AttributeRows);
