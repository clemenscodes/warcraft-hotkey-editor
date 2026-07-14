#[derive(Clone, PartialEq)]
pub struct FramedIconImageView {
    pub source: String,
    pub alt: String,
}

impl ddd::View for FramedIconImageView {}
