#[derive(Clone, PartialEq)]
pub struct ObjectIdView {
    pub text: String,
}

impl ddd::Layered for ObjectIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ObjectIdView {}
