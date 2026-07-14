pub mod components;
mod model;
mod presentation;
mod view;

pub use view::AttributesStatsView;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::attribute_rows::AttributeRows;
use dioxus::prelude::*;
use model::AttributesStatsModel;
use presentation::AttributeFigures;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AttributesStats(props: AttributesStatsModel) -> Element {
    let Some(hero) = props.hero else {
        return rsx! {};
    };
    let AttributeFigures {
        icon_src,
        icon_alt,
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
            StatIconFrame {
                src: icon_src,
                alt: icon_alt,
            }
            AttributeRows {
                strength,
                strength_is_primary,
                agility,
                agility_is_primary,
                intelligence,
                intelligence_is_primary,
            }
        }
    }
}

assert_component!(AttributesStats);
