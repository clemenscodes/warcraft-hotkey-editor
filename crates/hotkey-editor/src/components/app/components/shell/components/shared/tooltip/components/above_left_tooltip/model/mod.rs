use super::view::AboveLeftTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct AboveLeftTooltipModel {
    pub text: String,
}

impl From<&AboveLeftTooltipView> for AboveLeftTooltipModel {
    fn from(view: &AboveLeftTooltipView) -> Self {
        let AboveLeftTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AboveLeftTooltipModel {
    type View = AboveLeftTooltipView;
}
