use super::view::BelowCenterTooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BelowCenterTooltipModel {
    pub text: String,
}

impl From<&BelowCenterTooltipView> for BelowCenterTooltipModel {
    fn from(view: &BelowCenterTooltipView) -> Self {
        let BelowCenterTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for BelowCenterTooltipModel {
    type View = BelowCenterTooltipView;
}
