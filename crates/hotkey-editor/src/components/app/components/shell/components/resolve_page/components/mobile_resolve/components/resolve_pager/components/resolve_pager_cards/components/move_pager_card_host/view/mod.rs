use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

#[derive(Clone, PartialEq)]
pub struct MovePagerCardHostView {
    pub move_view: MoveView,
}

impl ddd::View for MovePagerCardHostView {}
