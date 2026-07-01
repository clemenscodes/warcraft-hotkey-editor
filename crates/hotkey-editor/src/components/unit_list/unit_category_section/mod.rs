mod logic;
mod props;
pub mod unit_category_heading;

use super::unit_card::UnitCard;
use dioxus::prelude::*;
use logic::UnitCategorySectionModel;
pub use props::UnitCategorySectionProps;
use unit_category_heading::UnitCategoryHeading;

/// One category of the unit list: a collapsible heading followed by the matching
/// unit cards. The heading, collapsed state, and cards are all built by
/// conversion in `logic`; this component is a thin renderer with no own class.
#[component]
pub fn UnitCategorySection(props: UnitCategorySectionProps) -> Element {
    let model = UnitCategorySectionModel::from(&props);
    rsx! {
        UnitCategoryHeading { ..model.heading }
        if !model.is_collapsed {
            for card in model.cards {
                UnitCard { key: "{card.unit_id}", ..card }
            }
        }
    }
}
