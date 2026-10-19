#[derive(Clone, PartialEq)]
pub struct TileIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for TileIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TileIconView {}
