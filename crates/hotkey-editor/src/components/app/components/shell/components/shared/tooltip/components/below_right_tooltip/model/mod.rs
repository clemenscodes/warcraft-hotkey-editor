use super::view::BelowRightTooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BelowRightTooltipModel {
    pub text: String,
}

impl From<&BelowRightTooltipView> for BelowRightTooltipModel {
    fn from(view: &BelowRightTooltipView) -> Self {
        let BelowRightTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for BelowRightTooltipModel {
    type View = BelowRightTooltipView;
}
