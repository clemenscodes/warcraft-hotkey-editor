pub mod components;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use super::shared::stat_rows::StatRows;
use components::attribute_row::{AttributeRow, AttributeRowProps};
use dioxus::prelude::*;
pub use props::{AttributesColumnProps, HeroDisplayData};

/// The hero attributes column: the primary-attribute icon beside the three
/// attribute rows.
#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let Some(hero) = props.hero else {
        return rsx! {};
    };
    let icon = StatIconFrameProps::from(&hero);
    let strength = AttributeRowProps::for_attribute(&hero, Attribute::Strength);
    let agility = AttributeRowProps::for_attribute(&hero, Attribute::Agility);
    let intelligence = AttributeRowProps::for_attribute(&hero, Attribute::Intelligence);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Attributes,
            with_icon: true,
            StatIconFrame { ..icon }
            StatRows {
                AttributeRow { ..strength }
                AttributeRow { ..agility }
                AttributeRow { ..intelligence }
            }
        }
    }
}

use props::Attribute;
