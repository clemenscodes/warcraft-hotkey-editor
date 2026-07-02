mod kinds;
mod logic;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use super::shared::stat_row::StatRow;
use super::shared::stat_rows::StatRows;
use dioxus::prelude::*;
use logic::AttributeRows;
pub use props::AttributesColumnProps;

/// The hero attributes column: the primary-attribute icon beside the three attribute
/// rows. Present only for a hero unit; an ordinary unit renders nothing here.
#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let Some(hero) = props.hero else {
        return rsx! {};
    };
    let icon = StatIconFrameProps::from(&hero);
    let AttributeRows {
        strength_row,
        agility_row,
        intelligence_row,
    } = AttributeRows::from(&hero);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Attributes,
            with_icon: true,
            StatIconFrame { ..icon }
            StatRows {
                StatRow { ..strength_row }
                StatRow { ..agility_row }
                StatRow { ..intelligence_row }
            }
        }
    }
}
