use dioxus::prelude::*;
use warcraft_api::SearchField;

/// The search-field toggle owns the field selection signal it reads and writes.
#[derive(Props, Clone, PartialEq)]
pub struct SearchFieldToggleProps {
    pub search_field: Signal<SearchField>,
}
