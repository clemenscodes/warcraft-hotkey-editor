#[derive(Clone, PartialEq)]
pub struct GridHeadingView {
    pub heading: &'static str,
}

impl ddd::View for GridHeadingView {}
