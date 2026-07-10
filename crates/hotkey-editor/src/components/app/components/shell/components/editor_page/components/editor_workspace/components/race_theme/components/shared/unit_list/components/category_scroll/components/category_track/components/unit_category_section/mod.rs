pub mod components;
mod hooks;
mod logic;
mod props;

use components::unit_card::UnitCard;
use components::unit_category_heading::UnitCategoryHeading;
use dioxus::prelude::*;
use hooks::use_unit_category_section;
pub use props::UnitCategorySectionProps;

/// One category of the unit list: a collapsible heading followed by the matching
/// unit cards. The heading, collapsed state, and cards are all built by the
/// composed hook (memoized on the catalog inputs); this component is a thin
/// renderer with no own class.
use tw_macro::assert_component;
assert_component!(UnitCategorySection);
#[component]
pub fn UnitCategorySection(props: UnitCategorySectionProps) -> Element {
    let model = use_unit_category_section(&props);
    rsx! {
        UnitCategoryHeading { ..model.heading }
        if !model.is_collapsed {
            for card in model.cards {
                UnitCard { key: "{card.unit_id.value()}", ..card }
            }
        }
    }
}
