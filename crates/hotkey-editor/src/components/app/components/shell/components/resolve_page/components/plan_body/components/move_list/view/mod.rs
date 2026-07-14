use crate::components::app::components::shell::components::resolve_page::presentation::MoveSection;

#[derive(Clone, PartialEq)]
pub struct MoveListView {
    pub section: Option<MoveSection>,
}

impl ddd::View for MoveListView {}
