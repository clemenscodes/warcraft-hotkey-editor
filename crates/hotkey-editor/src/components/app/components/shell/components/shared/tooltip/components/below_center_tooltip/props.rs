use super::view::BelowCenterTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct BelowCenterTooltipProps {
    pub text: String,
}

impl From<&BelowCenterTooltipView> for BelowCenterTooltipProps {
    fn from(view: &BelowCenterTooltipView) -> Self {
        let BelowCenterTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for BelowCenterTooltipProps {
    type View = BelowCenterTooltipView;
}
