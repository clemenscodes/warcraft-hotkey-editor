#[derive(Clone, PartialEq)]
pub struct AltStateLabelView {
    pub text: Option<String>,
}

impl ddd::View for AltStateLabelView {}
