#[derive(Clone, PartialEq)]
pub struct CardGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::Layered for CardGlowIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CardGlowIconView {}
