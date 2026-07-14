#[derive(Clone, PartialEq)]
pub struct TileIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for TileIconView {}
