use super::super::resolve_move_row::ResolveMoveRowProps;
use super::super::resolve_unresolved_row::ResolveUnresolvedRowProps;
use dioxus::prelude::*;

/// The active section's move cards, tagged with its category slug.
#[derive(Clone, PartialEq)]
pub struct ResolvePlanBodySection {
    pub data_category: &'static str,
    pub rows: Vec<ResolveMoveRowProps>,
}

/// The scrollable plan body: the active move section and the unresolved section.
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePlanBodyProps {
    pub section: Option<ResolvePlanBodySection>,
    pub unresolved_rows: Vec<ResolveUnresolvedRowProps>,
}
