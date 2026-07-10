use super::view::HelpLegendLabelView;
use dioxus::prelude::*;

/// The legend label's only input: the button name.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendLabelProps {
    #[props(into)]
    pub label: String,
}

impl From<&HelpLegendLabelView> for HelpLegendLabelProps {
    fn from(view: &HelpLegendLabelView) -> Self {
        let HelpLegendLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for HelpLegendLabelProps {
    type View = HelpLegendLabelView;
}
