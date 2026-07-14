use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

#[derive(Clone, PartialEq)]
pub struct UnresolvedMoveListView {
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for UnresolvedMoveListView {}
