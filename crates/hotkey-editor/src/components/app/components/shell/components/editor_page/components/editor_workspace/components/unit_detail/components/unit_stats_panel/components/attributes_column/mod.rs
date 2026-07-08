pub mod components;
mod logic;
mod props;
mod style;

use super::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use components::agility_row::AgilityRow;
use components::intelligence_row::IntelligenceRow;
use components::strength_row::StrengthRow;
use dioxus::prelude::*;
use logic::AttributeFigures;
pub use props::AttributesColumnProps;
use style::{CLASS, ROWS};
use tw_macro::assert_component;
assert_component!(AttributesColumn);

/// The hero attributes column: the primary-attribute icon beside the three attribute
/// rows, laid into the `attributes` grid area. Present only for a hero unit; an
/// ordinary unit renders nothing here. It names its rows directly — each attribute row
/// owns its own look and its primary glow.
#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let Some(hero) = props.hero else {
        return rsx! {};
    };
    let icon = StatIconFrameProps::from(&hero);
    let AttributeFigures {
        strength,
        strength_is_primary,
        agility,
        agility_is_primary,
        intelligence,
        intelligence_is_primary,
    } = AttributeFigures::from(&hero);
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..icon }
            div {
                class: ROWS,
                StrengthRow { statistic: strength, is_primary: strength_is_primary }
                AgilityRow { statistic: agility, is_primary: agility_is_primary }
                IntelligenceRow { statistic: intelligence, is_primary: intelligence_is_primary }
            }
        }
    }
}
