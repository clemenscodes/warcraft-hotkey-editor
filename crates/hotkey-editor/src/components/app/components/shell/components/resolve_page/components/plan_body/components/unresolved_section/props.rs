use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::unresolved_section::components::unresolved_move_list::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The unresolved-abilities section: one stuck card per ability the cascade could
/// not place.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedSectionProps {
    pub rows: Vec<UnresolvedRowProps>,
}
