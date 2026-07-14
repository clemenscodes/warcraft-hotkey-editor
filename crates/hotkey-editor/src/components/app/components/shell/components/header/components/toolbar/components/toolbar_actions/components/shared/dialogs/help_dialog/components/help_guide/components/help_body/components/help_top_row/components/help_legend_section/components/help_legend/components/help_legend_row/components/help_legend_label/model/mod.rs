use super::view::HelpLegendLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendLabelModel {
    #[props(into)]
    pub label: String,
}

impl From<&HelpLegendLabelView> for HelpLegendLabelModel {
    fn from(view: &HelpLegendLabelView) -> Self {
        let HelpLegendLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for HelpLegendLabelModel {
    type View = HelpLegendLabelView;
}
