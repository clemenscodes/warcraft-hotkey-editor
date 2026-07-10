use super::view::BelowRightTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct BelowRightTooltipProps {
    pub text: String,
}

impl From<&BelowRightTooltipView> for BelowRightTooltipProps {
    fn from(view: &BelowRightTooltipView) -> Self {
        let BelowRightTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for BelowRightTooltipProps {
    type View = BelowRightTooltipView;
}
