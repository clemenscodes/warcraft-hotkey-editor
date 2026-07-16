#[derive(Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnView {
    pub text: Option<String>,
}

impl ddd::View for AltStateHeaderLabelColumnView {}
