mod props;

use super::attribute_row::{AttributeRow, AttributeRowProps};
use super::stat_column::{StatColumn, StatColumnKind};
use super::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use super::stat_rows::StatRows;
use dioxus::prelude::*;
pub use props::{AttributesColumnProps, HeroDisplayData};

/// The hero attributes column: the primary-attribute icon beside the three
/// attribute rows.
#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let hero = props.hero;
    let icon = StatIconFrameProps::from(&hero);
    let strength = AttributeRowProps::from((&hero, Attribute::Strength));
    let agility = AttributeRowProps::from((&hero, Attribute::Agility));
    let intelligence = AttributeRowProps::from((&hero, Attribute::Intelligence));
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
