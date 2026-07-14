#[derive(Clone, PartialEq)]
pub struct ResolvePageView {
    pub entry: Option<String>,
}

impl ddd::View for ResolvePageView {}
