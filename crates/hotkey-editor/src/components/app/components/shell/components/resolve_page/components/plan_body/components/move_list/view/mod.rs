use crate::components::app::components::shell::components::resolve_page::presentation::MoveSection;

/// The published `View` contract mirroring [`MoveListModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveListView {
    pub section: Option<MoveSection>,
}

impl ddd::View for MoveListView {}
