use super::view::StatLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&StatLabelView> for StatLabelModel {
    fn from(view: &StatLabelView) -> Self {
        let StatLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for StatLabelModel {
    type View = StatLabelView;
}
