#[derive(Clone, PartialEq)]
pub struct CollisionKindTabCountView {
    pub count: usize,
}

impl ddd::Layered for CollisionKindTabCountView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CollisionKindTabCountView {}
