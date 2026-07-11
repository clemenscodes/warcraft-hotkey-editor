use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

/// The published `View` contract mirroring [`MoveRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveRowView {
    pub move_view: MoveView,
}

impl ddd::View for MoveRowView {}
