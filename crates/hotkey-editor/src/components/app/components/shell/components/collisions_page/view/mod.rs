#[derive(Clone, PartialEq)]
pub struct CollisionsPageView {
    pub kind: Option<String>,
    pub entry: Option<String>,
}

impl ddd::View for CollisionsPageView {}
