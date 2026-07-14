#[derive(Clone, PartialEq)]
pub struct PlaceholderIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for PlaceholderIconView {}
