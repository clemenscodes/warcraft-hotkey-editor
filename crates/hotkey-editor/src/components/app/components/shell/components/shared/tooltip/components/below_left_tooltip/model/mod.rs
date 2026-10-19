use super::view::BelowLeftTooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BelowLeftTooltipModel {
    pub text: String,
}

impl From<&BelowLeftTooltipView> for BelowLeftTooltipModel {
    fn from(view: &BelowLeftTooltipView) -> Self {
        let BelowLeftTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for BelowLeftTooltipModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BelowLeftTooltipModel {
    type View = BelowLeftTooltipView;
}
