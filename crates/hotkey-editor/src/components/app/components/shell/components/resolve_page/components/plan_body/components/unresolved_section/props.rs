use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The unresolved-abilities section: one stuck card per ability the cascade could
/// not place.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedSectionProps {
    pub unresolved: Vec<UnresolvedView>,
}
