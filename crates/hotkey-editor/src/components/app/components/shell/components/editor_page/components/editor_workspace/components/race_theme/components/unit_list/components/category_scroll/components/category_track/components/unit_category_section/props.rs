use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// One collapsible category of units in the sidebar list, identified by its kind. Its
/// label and collapsed state, the catalog query that fills it, and the selection its
/// cards drive are all read from context by the section's hook, so the section is fed
/// only which kind it is.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategorySectionProps {
    pub category_kind: UnitKind,
}
