#[derive(Clone, PartialEq)]
pub struct InfoPopoverView {
    pub text: &'static str,
}

impl ddd::View for InfoPopoverView {}
