#[derive(Clone, PartialEq)]
pub struct RegenQualifierView {
    pub text: Option<&'static str>,
}

impl ddd::View for RegenQualifierView {}
