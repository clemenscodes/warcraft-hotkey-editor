use super::view::HelpLegendIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendIconModel {
    pub icon: &'static str,
}

impl From<&HelpLegendIconView> for HelpLegendIconModel {
    fn from(view: &HelpLegendIconView) -> Self {
        let HelpLegendIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Model for HelpLegendIconModel {
    type View = HelpLegendIconView;
}
