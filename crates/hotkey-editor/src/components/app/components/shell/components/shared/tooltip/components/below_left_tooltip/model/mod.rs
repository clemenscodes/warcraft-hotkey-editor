use super::view::BelowLeftTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
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

impl ddd::Model for BelowLeftTooltipModel {
    type View = BelowLeftTooltipView;
}
