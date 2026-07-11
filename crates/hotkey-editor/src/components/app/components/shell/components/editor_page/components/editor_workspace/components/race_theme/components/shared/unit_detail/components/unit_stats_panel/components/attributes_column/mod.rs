pub mod components;
mod model;
mod presentation;
mod view;

pub use view::AttributesColumnView;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::attribute_rows::AttributeRows;
use dioxus::prelude::*;
use model::AttributesColumnModel;
use presentation::AttributeFigures;
use style::CLASS;
use tw_macro::assert_component;

/// The hero attributes column: the primary-attribute icon beside the three attribute
/// rows, laid into the `attributes` grid area. Present only for a hero unit; an
/// ordinary unit renders nothing here.
#[component]
pub fn AttributesColumn(props: AttributesColumnModel) -> Element {
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
            StatIconFrame { src: icon_src, alt: icon_alt }
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

assert_component!(AttributesColumn);
