pub mod components;
mod model;
mod style;
mod view;

pub use view::SearchDialogFiltersView;

use components::mode_chip_row::ModeChipRow;
use components::race_chip_row::RaceChipRow;
use dioxus::prelude::*;
use model::SearchDialogFiltersModel;
use style::CLASS;
use tw_macro::assert_component;

/// Every filter in one block, because together they are one filter.
///
/// Race, mode and visibility are all toggles feeding the same
/// [`UnitFilterQuery`](crate::services::unit_catalog::queries::unit_filter_query::UnitFilterQuery),
/// so they belong side by side and above the search: you narrow what you are
/// looking through before you type what you are looking for, not after.
#[component]
pub fn SearchDialogFilters(props: SearchDialogFiltersModel) -> Element {
    let SearchDialogFiltersModel {} = props;
    rsx! {
        div {
            class: CLASS,
            RaceChipRow {}
            ModeChipRow {}
        }
    }
}

assert_component!(SearchDialogFilters);
