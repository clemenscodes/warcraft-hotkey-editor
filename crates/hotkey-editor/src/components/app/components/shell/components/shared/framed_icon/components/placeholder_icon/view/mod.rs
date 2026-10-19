#[derive(Clone, PartialEq)]
pub struct PlaceholderIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::Layered for PlaceholderIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlaceholderIconView {}
