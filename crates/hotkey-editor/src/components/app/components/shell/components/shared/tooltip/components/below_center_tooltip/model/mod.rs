use super::view::BelowCenterTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
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
