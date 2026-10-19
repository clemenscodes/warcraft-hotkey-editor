use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

#[derive(Clone, PartialEq)]
pub struct MovePanelView {
    pub move_view: MoveView,
}

impl ddd::Layered for MovePanelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MovePanelView {}
