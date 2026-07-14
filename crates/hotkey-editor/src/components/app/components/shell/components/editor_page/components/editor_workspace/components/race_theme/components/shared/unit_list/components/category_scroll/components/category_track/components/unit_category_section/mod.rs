pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitCategorySectionView;

use components::unit_card::UnitCard;
use components::unit_category_heading::UnitCategoryHeading;
use dioxus::prelude::*;
use model::UnitCategorySectionModel;
use presentation::use_unit_category_section;
use tw_macro::assert_component;

#[component]
pub fn UnitCategorySection(props: UnitCategorySectionModel) -> Element {
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
