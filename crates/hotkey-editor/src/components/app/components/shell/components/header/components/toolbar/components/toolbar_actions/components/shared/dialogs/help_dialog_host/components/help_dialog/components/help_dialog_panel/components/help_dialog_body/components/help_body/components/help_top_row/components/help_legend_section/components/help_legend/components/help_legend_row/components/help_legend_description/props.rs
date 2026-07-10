use super::view::HelpLegendDescriptionView;
use dioxus::prelude::*;

/// The legend description's only input: the copy.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendDescriptionProps {
    #[props(into)]
    pub description: String,
}

impl From<&HelpLegendDescriptionView> for HelpLegendDescriptionProps {
    fn from(view: &HelpLegendDescriptionView) -> Self {
        let HelpLegendDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Props for HelpLegendDescriptionProps {
    type View = HelpLegendDescriptionView;
}
