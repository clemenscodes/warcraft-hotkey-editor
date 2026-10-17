#[derive(Clone, PartialEq)]
pub struct IdleKeycapView {
    pub label: String,
}

impl ddd::Layered for IdleKeycapView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleKeycapView {}
