use super::view::AboveRightTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct AboveRightTooltipProps {
    pub text: String,
}

impl From<&AboveRightTooltipView> for AboveRightTooltipProps {
    fn from(view: &AboveRightTooltipView) -> Self {
        let AboveRightTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AboveRightTooltipProps {
    type View = AboveRightTooltipView;
}
