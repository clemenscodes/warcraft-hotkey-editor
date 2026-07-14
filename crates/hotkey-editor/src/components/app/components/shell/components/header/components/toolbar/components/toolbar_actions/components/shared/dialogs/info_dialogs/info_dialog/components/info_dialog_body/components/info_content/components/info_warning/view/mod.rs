#[derive(Clone, PartialEq)]
pub struct InfoWarningView {
    pub warning: Option<&'static str>,
}

impl ddd::View for InfoWarningView {}
