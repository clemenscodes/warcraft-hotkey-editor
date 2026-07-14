#[derive(Clone, PartialEq)]
pub struct TileGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for TileGlowIconView {}
