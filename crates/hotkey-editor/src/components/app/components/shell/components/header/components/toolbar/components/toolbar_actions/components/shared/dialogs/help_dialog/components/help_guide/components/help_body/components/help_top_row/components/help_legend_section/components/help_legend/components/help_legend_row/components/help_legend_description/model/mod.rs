use super::view::HelpLegendDescriptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendDescriptionModel {
    #[props(into)]
    pub description: String,
}

impl From<&HelpLegendDescriptionView> for HelpLegendDescriptionModel {
    fn from(view: &HelpLegendDescriptionView) -> Self {
        let HelpLegendDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Model for HelpLegendDescriptionModel {
    type View = HelpLegendDescriptionView;
}
