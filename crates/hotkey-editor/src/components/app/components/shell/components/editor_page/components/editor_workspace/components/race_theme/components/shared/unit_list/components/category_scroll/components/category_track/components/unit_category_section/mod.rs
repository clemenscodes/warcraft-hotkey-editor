pub mod components;
mod hooks;
mod logic;
mod props;

use components::unit_card::UnitCard;
use components::unit_category_heading::UnitCategoryHeading;
use dioxus::prelude::*;
use hooks::use_unit_category_section;
use props::UnitCategorySectionProps;
use tw_macro::assert_component;

/// One category of the unit list: a collapsible heading followed by the matching
/// unit cards. The heading, collapsed state, and cards are all built by the
/// composed hook (memoized on the catalog inputs); this component is a thin
/// renderer with no own class.
#[component]
pub fn UnitCategorySection(props: UnitCategorySectionProps) -> Element {
    let model = use_unit_category_section(&props);
    let heading = model.heading;
    let is_collapsed = model.is_collapsed;
    let cards = model.cards;
    rsx! {
        UnitCategoryHeading {
            label: heading.label,
            is_collapsed: heading.is_collapsed,
            on_toggle: heading.on_toggle,
        }
        if !is_collapsed {
            for entry in cards {
                UnitCard {
                    key: "{entry.unit_id.value()}",
                    unit_id: entry.unit_id,
                    display_name: entry.display_name.clone(),
                    icon_path: entry.icon_path.clone(),
                    unit_kind: entry.unit_kind,
                }
            }
        }
    }
}

assert_component!(UnitCategorySection);
