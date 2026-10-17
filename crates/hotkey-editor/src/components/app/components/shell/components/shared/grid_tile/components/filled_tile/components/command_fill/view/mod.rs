#[derive(Clone, PartialEq)]
pub struct CommandFillView {
    pub active: bool,
}

impl ddd::Layered for CommandFillView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CommandFillView {}
