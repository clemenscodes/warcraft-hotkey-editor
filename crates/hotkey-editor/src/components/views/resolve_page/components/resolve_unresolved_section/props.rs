use crate::components::views::resolve_page::components::resolve_unresolved_row::ResolveUnresolvedRowProps;
use dioxus::prelude::*;

/// The unresolved-abilities section: one stuck card per ability the cascade could
/// not place.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveUnresolvedSectionProps {
    pub rows: Vec<ResolveUnresolvedRowProps>,
}
