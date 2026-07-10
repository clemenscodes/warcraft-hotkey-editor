use super::view::AboveLeftTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct AboveLeftTooltipProps {
    pub text: String,
}

impl From<&AboveLeftTooltipView> for AboveLeftTooltipProps {
    fn from(view: &AboveLeftTooltipView) -> Self {
        let AboveLeftTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AboveLeftTooltipProps {
    type View = AboveLeftTooltipView;
}
