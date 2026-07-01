pub mod components;
mod props;
mod style;

use dioxus::prelude::*;
use warcraft_database::SearchField;

use crate::assert_component;
use components::search_field_button::SearchFieldButton;
use style::CLASS;

pub use props::SearchFieldToggleProps;

assert_component!(SearchFieldToggle);

/// The Unit/Ability toggle that chooses which field the search box matches against.
#[component]
pub fn SearchFieldToggle(props: SearchFieldToggleProps) -> Element {
    let mut search_field = props.search_field;
    let current = *search_field.read();
    let select_unit = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::UnitName);
    });
    let select_ability = EventHandler::new(move |_event: MouseEvent| {
        search_field.set(SearchField::Ability);
    });
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Search by",
            SearchFieldButton {
                label: "Unit",
                is_active: current == SearchField::UnitName,
                on_select: select_unit,
            }
            SearchFieldButton {
                label: "Ability",
                is_active: current == SearchField::Ability,
                on_select: select_ability,
            }
        }
    }
}
