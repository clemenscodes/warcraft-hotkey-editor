use super::view::BelowLeftTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct BelowLeftTooltipProps {
    pub text: String,
}

impl From<&BelowLeftTooltipView> for BelowLeftTooltipProps {
    fn from(view: &BelowLeftTooltipView) -> Self {
        let BelowLeftTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for BelowLeftTooltipProps {
    type View = BelowLeftTooltipView;
}
