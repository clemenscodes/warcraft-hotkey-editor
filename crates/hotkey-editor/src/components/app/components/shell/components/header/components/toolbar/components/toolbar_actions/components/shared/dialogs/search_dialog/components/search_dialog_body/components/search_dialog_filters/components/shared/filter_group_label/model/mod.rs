use super::view::FilterGroupLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterGroupLabelModel {
    pub label: &'static str,
}

impl From<&FilterGroupLabelView> for FilterGroupLabelModel {
    fn from(view: &FilterGroupLabelView) -> Self {
        let FilterGroupLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for FilterGroupLabelModel {
    type View = FilterGroupLabelView;
}
