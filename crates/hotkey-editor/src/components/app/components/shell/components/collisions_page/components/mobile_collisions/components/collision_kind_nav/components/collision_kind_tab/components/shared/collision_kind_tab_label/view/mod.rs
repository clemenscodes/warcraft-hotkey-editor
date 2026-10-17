#[derive(Clone, PartialEq)]
pub struct CollisionKindTabLabelView {
    pub text: String,
}

impl ddd::Layered for CollisionKindTabLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CollisionKindTabLabelView {}
