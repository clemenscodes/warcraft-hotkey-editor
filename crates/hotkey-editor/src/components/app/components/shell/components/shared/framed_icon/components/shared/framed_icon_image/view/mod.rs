#[derive(Clone, PartialEq)]
pub struct FramedIconImageView {
    pub source: String,
    pub alt: String,
}

impl ddd::Layered for FramedIconImageView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FramedIconImageView {}
