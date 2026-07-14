#[derive(Clone, PartialEq)]
pub struct TilePlainIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for TilePlainIconView {}
