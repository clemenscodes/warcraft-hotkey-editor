#[derive(Clone, PartialEq)]
pub struct TileGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::Layered for TileGlowIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TileGlowIconView {}
