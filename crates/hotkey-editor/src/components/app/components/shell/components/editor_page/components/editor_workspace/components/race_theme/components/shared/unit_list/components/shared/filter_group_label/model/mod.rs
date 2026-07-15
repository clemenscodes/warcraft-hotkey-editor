use super::view::FilterGroupLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterGroupLabelModel {
    pub text: &'static str,
}

impl From<&FilterGroupLabelView> for FilterGroupLabelModel {
    fn from(view: &FilterGroupLabelView) -> Self {
        let FilterGroupLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for FilterGroupLabelModel {
    type View = FilterGroupLabelView;
}
