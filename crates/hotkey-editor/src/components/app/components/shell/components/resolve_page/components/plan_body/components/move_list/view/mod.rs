use crate::components::app::components::shell::components::resolve_page::presentation::MoveSection;

#[derive(Clone, PartialEq)]
pub struct MoveListView {
    pub section: Option<MoveSection>,
}

impl ddd::Layered for MoveListView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MoveListView {}
