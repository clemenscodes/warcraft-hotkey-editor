#[derive(Clone, PartialEq)]
pub struct TilePlainIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::Layered for TilePlainIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TilePlainIconView {}
