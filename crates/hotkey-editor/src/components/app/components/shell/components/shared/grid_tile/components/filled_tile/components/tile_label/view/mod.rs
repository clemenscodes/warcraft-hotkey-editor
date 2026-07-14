#[derive(Clone, PartialEq)]
pub struct TileLabelView {
    pub text: Option<String>,
}

impl ddd::View for TileLabelView {}
