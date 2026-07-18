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
/// [`UnitFilterQuery`](crate::services::unit_catalog::queries::unit_filter_query::UnitFilterQuery).
/// The block is folded away by default behind the config button and renders
/// nothing when closed, so the search field stays the first thing the dialog
/// shows. It is a guarded child, it decides to draw nothing rather than letting a
/// parent branch on `open`.
#[component]
pub fn SearchDialogFilters(props: SearchDialogFiltersModel) -> Element {
    if !props.open {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            RaceChipRow {}
            ModeChipRow {}
        }
    }
}

assert_component!(SearchDialogFilters);
