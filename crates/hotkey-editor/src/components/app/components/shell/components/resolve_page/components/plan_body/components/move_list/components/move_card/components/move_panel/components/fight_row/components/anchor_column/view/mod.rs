use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;

#[derive(Clone, PartialEq)]
pub struct AnchorColumnView {
    pub move_view: MoveView,
}

impl ddd::Layered for AnchorColumnView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AnchorColumnView {}
