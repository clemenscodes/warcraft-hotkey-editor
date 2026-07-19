#[derive(Clone, PartialEq)]
pub struct FilterGroupLabelView {
    pub label: &'static str,
}

impl ddd::View for FilterGroupLabelView {}
