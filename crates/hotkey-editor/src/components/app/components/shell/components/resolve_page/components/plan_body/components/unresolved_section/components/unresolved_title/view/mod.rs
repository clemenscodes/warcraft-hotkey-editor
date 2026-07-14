#[derive(Clone, PartialEq)]
pub struct UnresolvedTitleView {
    pub text: &'static str,
}

impl ddd::View for UnresolvedTitleView {}
