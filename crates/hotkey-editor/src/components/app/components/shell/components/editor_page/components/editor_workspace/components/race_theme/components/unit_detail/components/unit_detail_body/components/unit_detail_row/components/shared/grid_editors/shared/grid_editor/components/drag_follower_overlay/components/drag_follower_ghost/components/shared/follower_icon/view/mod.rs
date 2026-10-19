#[derive(Clone, PartialEq)]
pub struct FollowerIconView {
    pub src: String,
    pub alt: String,
}

impl ddd::Layered for FollowerIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FollowerIconView {}
