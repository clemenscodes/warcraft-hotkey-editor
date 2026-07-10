pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::unit_list_search_icon::UnitListSearchIcon;
use components::unit_list_search_input::{UnitListSearchInput, UnitListSearchInputProps};
use style::CLASS;
use tw_macro::assert_component;

pub use props::UnitListSearchProps;

/// The unit list's search box: a magnifier icon (mobile only) over the query input.
#[component]
pub fn UnitListSearch(props: UnitListSearchProps) -> Element {
    let input = UnitListSearchInputProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitListSearchIcon {}
            UnitListSearchInput { ..input }
        }
    }
}

assert_component!(UnitListSearch);
